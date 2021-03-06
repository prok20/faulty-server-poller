use async_trait::async_trait;

pub use polling_service_impl::PollingServiceImpl;

use crate::polling::dto::{Run, RunId, StartRunRequestDto, StartRunResponseDto};
use crate::polling::errors::ServiceResult;

mod polling_service_impl;

#[cfg_attr(test, mockall::automock)]
#[async_trait(? Send)]
pub trait PollingService {
    async fn start_run(
        &self,
        start_run_request_dto: StartRunRequestDto,
    ) -> ServiceResult<StartRunResponseDto>;
    async fn get_run(&self, run_id: RunId) -> ServiceResult<Run>;
}
