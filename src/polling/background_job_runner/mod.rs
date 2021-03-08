mod tokio_background_job_runner;

pub use tokio_background_job_runner::TokioBackgroundJobRunner;

use crate::polling::dto::RunJob;
use crate::polling::errors::ServiceResult;
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait BackgroundJobRunner {
    async fn try_push_job(&self, run_job: RunJob) -> ServiceResult<()>;
}
