use crate::polling::background_job_runner::BackgroundJobRunner;
use crate::polling::dto::{NewRun, Run, RunId, RunJob, StartRunRequestDto, StartRunResponseDto};
use crate::polling::errors::ServiceResult;
use crate::polling::polling_service::PollingService;
use crate::polling::run_repository::RunRepository;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct PollingServiceImpl<R, J> {
    run_repo: R,
    job_runner: J,
}

#[async_trait(? Send)]
impl<R, J> PollingService for PollingServiceImpl<R, J>
where
    R: RunRepository,
    J: BackgroundJobRunner,
{
    async fn start_run(
        &self,
        start_run_request_dto: StartRunRequestDto,
    ) -> ServiceResult<StartRunResponseDto> {
        let id = self.run_repo.generate_run_id().await;

        self.job_runner
            .try_push_job(RunJob {
                id,
                duration: std::time::Duration::from_secs(start_run_request_dto.seconds),
            })
            .await?;

        self.run_repo
            .save_run(&NewRun {
                id,
                seconds: start_run_request_dto.seconds,
            })
            .await?;

        Ok(StartRunResponseDto { id })
    }

    async fn get_run(&self, run_id: RunId) -> ServiceResult<Run> {
        self.run_repo.get_run_by_id(run_id).await
    }
}

impl<R, J> PollingServiceImpl<R, J>
where
    R: RunRepository,
    J: BackgroundJobRunner,
{
    #[allow(dead_code)]
    pub fn new(run_repo: R, job_runner: J) -> Self {
        Self {
            run_repo,
            job_runner,
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use crate::polling::background_job_runner::MockBackgroundJobRunner;
    use crate::polling::dto::RunStatus;
    use crate::polling::run_repository::MockRunRepository;
    use mockall::predicate::eq;

    #[actix_rt::test]
    async fn start_run_correctly() {
        let id = RunId::new_v4();
        let request = StartRunRequestDto { seconds: 15 };

        let run_repo = {
            let mut r = MockRunRepository::new();
            r.expect_generate_run_id().return_const(id);
            r.expect_save_run()
                .with(eq(NewRun {
                    id,
                    seconds: request.seconds,
                }))
                .return_const(ServiceResult::Ok(()));
            r
        };
        let job_runner = {
            let mut j = MockBackgroundJobRunner::new();
            j.expect_try_push_job()
                .with(eq(RunJob {
                    id,
                    duration: std::time::Duration::from_secs(request.seconds),
                }))
                .return_const(ServiceResult::Ok(()));
            j
        };

        let service = PollingServiceImpl::new(run_repo, job_runner);

        let actual_result = service.start_run(request).await;
        assert_eq!(Ok(StartRunResponseDto { id }), actual_result)
    }

    #[actix_rt::test]
    async fn get_run_correctly() {
        let id = RunId::new_v4();
        let expected_result = ServiceResult::Ok(Run {
            id,
            status: RunStatus::Finished,
            successful_responses_count: 10,
            sum: 150,
        });

        let run_repo = {
            let mut r = MockRunRepository::new();
            r.expect_generate_run_id().return_const(id);
            r.expect_get_run_by_id()
                .with(eq(id))
                .return_const(expected_result.clone());
            r
        };
        let job_runner = MockBackgroundJobRunner::new();

        let service = PollingServiceImpl::new(run_repo, job_runner);

        let actual_result = service.get_run(id).await;
        assert_eq!(expected_result, actual_result)
    }
}
