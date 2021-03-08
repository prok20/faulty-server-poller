#[cfg(test)]
use mockall::mock;

use crate::polling::dto::{FaultyServerResponse, RunId};
use async_trait::async_trait;

mod reqwest_request_sender;
pub use reqwest_request_sender::ReqwestRequestSender;

#[async_trait]
pub trait RequestSender: Clone + Send + Sync {
    async fn send_request(&self, id: RunId) -> FaultyServerResponse;
}

#[cfg(test)]
mock! {
    pub RequestSender {}

    impl Clone for RequestSender {
        fn clone(&self) -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl RequestSender for RequestSender {
        async fn send_request(&self, id: RunId) -> FaultyServerResponse;
    }
}
