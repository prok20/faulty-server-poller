use std::time::Duration;
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

#[derive(Debug, Clone, PartialEq)]
pub struct RunJob {
    pub id: RunId,
    pub duration: Duration,
}

pub struct RunJobResult {
    pub id: RunId,
    pub successful_responses: u64,
    pub value_sum: u64,
}
