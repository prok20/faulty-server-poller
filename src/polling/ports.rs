use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

pub type RunId = Uuid;

pub struct NewRun {
    pub id: RunId,
    pub seconds: u64,
}

pub enum RunStatus {
    Finished,
    InProgress,
}

pub struct Run {
    pub id: RunId,
    pub status: RunStatus,
    pub successful_responses_count: u64,
    pub sum: u64,
}

#[async_trait]
pub trait RunRepo {
    async fn generate_run_id(&self) -> Result<RunId>;
    async fn save_run(&self, run: &NewRun) -> Result<()>;
    async fn update_run(&self, run: &Run) -> Result<()>;
    async fn get_run(&self, run_id: RunId) -> Result<Run>;
}

#[async_trait]
pub trait PollingService {
    async fn start_run(&self) -> Result<RunId>;
    async fn get_run(&self, run_id: RunId) -> Result<Run>;
}
