use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use futures::{stream, StreamExt};

use crate::configuration::settings::PollingSettings;
use crate::polling::background_job_runner::BackgroundJobRunner;
use crate::polling::dto::{FaultyServerResponse, Run, RunJob, RunJobResult, RunStatus};
use crate::polling::errors::ServiceResult;
use crate::polling::request_sender::RequestSender;
use crate::polling::run_repository::RunRepository;

#[derive(Clone, Debug)]
pub struct TokioBackgroundJobRunner<R, S> {
    tx: Sender<RunJob>,
    // required because we do not use request sender in struct itself
    request_sender_type: PhantomData<S>,
    run_repo_type: PhantomData<R>,
}

impl<R, S> TokioBackgroundJobRunner<R, S>
where
    R: RunRepository + 'static,
    S: RequestSender + 'static,
{
    pub async fn new(run_repo: R, request_sender: S, settings: PollingSettings) -> Self {
        let (tx, rx) = async_channel::bounded(settings.max_pending_runs.max(1));
        {
            std::thread::spawn(move || Self::init_runtime(run_repo, rx, request_sender, settings));
        }

        Self {
            tx,
            request_sender_type: PhantomData,
            run_repo_type: PhantomData,
        }
    }

    #[tokio::main]
    async fn init_runtime(
        run_repo: R,
        rx: Receiver<RunJob>,
        request_sender: S,
        settings: PollingSettings,
    ) {
        for _ in 0..settings.max_concurrent_runs - 1 {
            let run_repo = run_repo.clone();
            let settings = settings.clone();
            let rx = rx.clone();
            let request_sender = request_sender.clone();

            tokio::spawn(async move {
                Self::process_run_jobs(run_repo, rx, request_sender, settings).await;
            });
        }
        Self::process_run_jobs(run_repo, rx, request_sender, settings).await;
    }

    async fn process_run_jobs(
        run_repo: R,
        rx: Receiver<RunJob>,
        request_sender: S,
        settings: PollingSettings,
    ) {
        loop {
            let job = rx.recv().await.expect("Channel is closed");

            let result =
                Self::execute_job(job, &request_sender, settings.concurrent_requests_per_run).await;

            run_repo
                .update_run(&Run {
                    id: result.id,
                    status: RunStatus::Finished,
                    successful_responses_count: result.successful_responses,
                    sum: result.value_sum,
                })
                .await
                .expect("Failed to update run in repository");
        }
    }

    async fn execute_job(
        job: RunJob,
        request_sender: &S,
        concurrent_requests_per_run: usize,
    ) -> RunJobResult {
        let value_sum = Arc::new(Mutex::new(0u64));
        let successful_responses = Arc::new(Mutex::new(0u64));

        let requests = stream::repeat(())
            .map(|_| {
                let id = job.id;
                async move { request_sender.send_request(id).await }
            })
            .buffer_unordered(concurrent_requests_per_run);

        let fut = requests.for_each_concurrent(None, |response| async {
            let successful_responses = Arc::clone(&successful_responses);
            let value_sum = Arc::clone(&value_sum);

            async move {
                if let FaultyServerResponse::Ok { value } = response {
                    *successful_responses.lock().unwrap() += 1;
                    *value_sum.lock().unwrap() += value as u64;
                }
            }
            .await
        });

        tokio::time::timeout(job.duration, fut)
            .await
            .expect_err("Run finished earlier than expected timeout");

        let successful_responses = *successful_responses.lock().unwrap();
        let value_sum = *value_sum.lock().unwrap();

        RunJobResult {
            id: job.id,
            successful_responses,
            value_sum,
        }
    }
}

#[async_trait(? Send)]
impl<R: RunRepository, S: RequestSender> BackgroundJobRunner for TokioBackgroundJobRunner<R, S> {
    async fn try_push_job(&self, run_job: RunJob) -> ServiceResult<()> {
        Ok(self.tx.try_send(run_job)?)
    }
}

#[cfg(test)]
mod should {
    use std::str::FromStr;

    use super::*;
    use crate::polling::dto::RunId;
    use crate::polling::errors::ServiceError;
    use crate::polling::request_sender::MockRequestSender;
    use crate::polling::run_repository::MockRunRepository;
    use tokio::time::sleep;

    fn mock_run_repo() -> MockRunRepository {
        let mut r = MockRunRepository::new();
        r.expect_clone().returning(MockRunRepository::new);
        r
    }

    fn mock_request_sender() -> MockRequestSender {
        let mut r = MockRequestSender::new();
        r.expect_clone().returning(|| {
            let mut r = MockRequestSender::new();
            r.expect_send_request()
                .return_const(FaultyServerResponse::Ok { value: 50 });
            r
        });
        r.expect_send_request()
            .return_const(FaultyServerResponse::Ok { value: 50 });
        r
    }

    #[actix_rt::test]
    async fn successfully_execute_single_job() {
        let job = RunJob {
            id: RunId::new_v4(),
            duration: std::time::Duration::from_secs(3),
        };

        let run_repo = {
            let mut r = mock_run_repo();
            let expected_id = job.id;
            r.expect_update_run()
                .withf(move |r| {
                    r.id == expected_id
                        && r.status == RunStatus::Finished
                        && r.successful_responses_count > 0
                        && r.sum > 0
                })
                .return_const(ServiceResult::Ok(()));
            r
        };
        let request_sender = mock_request_sender();
        let settings = PollingSettings {
            polling_address: "127.0.0.1:0".into(),
            max_concurrent_runs: 3,
            max_pending_runs: 3,
            concurrent_requests_per_run: 3,
        };

        let runner = TokioBackgroundJobRunner::new(run_repo, request_sender, settings).await;

        let actual_result = runner.try_push_job(job).await;
        sleep(std::time::Duration::from_secs(4)).await;
        assert_eq!(Ok(()), actual_result);
    }

    #[actix_rt::test]
    async fn return_too_many_requests_error_when_exceeds_run_concurrency() {
        let run_repo = mock_run_repo();
        let request_sender = mock_request_sender();
        let settings = PollingSettings {
            polling_address: "127.0.0.1:0".into(),
            max_concurrent_runs: 1,
            max_pending_runs: 1,
            concurrent_requests_per_run: 3,
        };

        let runner = TokioBackgroundJobRunner::new(run_repo, request_sender, settings).await;

        let job = RunJob {
            id: RunId::from_str("247fe111-0018-485e-9971-66cb27308221").unwrap(),
            duration: std::time::Duration::from_secs(10),
        };
        runner.try_push_job(job.clone()).await.unwrap();
        sleep(std::time::Duration::from_secs(1)).await;
        runner.try_push_job(job.clone()).await.unwrap();
        sleep(std::time::Duration::from_secs(1)).await;
        let actual_result = runner.try_push_job(job).await;
        assert_eq!(Err(ServiceError::TooManyRequests), actual_result);
    }
}
