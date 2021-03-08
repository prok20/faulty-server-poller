use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;

use crate::polling::dto::{NewRun, Run, RunId};
use crate::polling::errors::ServiceResult;

#[async_trait]
pub trait RunRepository: Clone + Send + Sync {
    async fn generate_run_id(&self) -> RunId;
    async fn save_run(&self, run: &NewRun) -> ServiceResult<()>;
    async fn update_run(&self, run: &Run) -> ServiceResult<()>;
    async fn get_run(&self, run_id: RunId) -> ServiceResult<Run>;
}

#[cfg(test)]
mock! {
    pub RunRepository {}

    impl Clone for RunRepository {
        fn clone(&self) -> Self;
    }

    #[async_trait]
    impl RunRepository for RunRepository {
        async fn generate_run_id(&self) -> RunId;
        async fn save_run(&self, run: &NewRun) -> ServiceResult<()>;
        async fn update_run(&self, run: &Run) -> ServiceResult<()>;
        async fn get_run(&self, run_id: RunId) -> ServiceResult<Run>;
    }
}
