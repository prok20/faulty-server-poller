use crate::polling::errors::ServiceResult;
use async_trait::async_trait;
use uuid::Uuid;

pub type RunId = Uuid;

pub struct NewRun {
    pub id: RunId,
    pub seconds: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RunStatus {
    Finished,
    InProgress,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StartRunRequestDto {
    pub seconds: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StartRunResponseDto {
    pub id: RunId,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Run {
    pub id: RunId,
    pub status: RunStatus,
    pub successful_responses_count: u64,
    pub sum: u64,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum FaultyServerResponse {
    Ok { value: u32 },
    Err { error: String },
}

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait RunRepo {
    async fn generate_run_id(&self) -> RunId;
    async fn save_run(&self, run: &NewRun) -> ServiceResult<()>;
    async fn update_run(&self, run: &Run) -> ServiceResult<()>;
    async fn get_run(&self, run_id: RunId) -> ServiceResult<Run>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait PollingService {
    async fn start_run(
        &self,
        start_run_request_dto: StartRunRequestDto,
    ) -> ServiceResult<StartRunResponseDto>;
    async fn get_run(&self, run_id: RunId) -> ServiceResult<Run>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait(?Send)]
pub trait RequestSender {
    async fn send_request(&self) -> FaultyServerResponse;
}
