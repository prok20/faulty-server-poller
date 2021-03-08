use crate::polling::dto::{FaultyServerResponse, RunId};
use crate::polling::request_sender::RequestSender;
use async_trait::async_trait;
use reqwest::Client;

#[derive(Clone)]
pub struct ReqwestRequestSender {
    client: Client,
    polling_address: String,
}

impl ReqwestRequestSender {
    pub fn new(client: Client, polling_address: String) -> Self {
        Self {
            client,
            polling_address,
        }
    }
}

#[async_trait]
impl RequestSender for ReqwestRequestSender {
    async fn send_request(&self, id: RunId) -> FaultyServerResponse {
        self.client
            .get(&self.polling_address)
            .header("X-Run-Id", id.to_string())
            .send()
            .await
            .expect("Failed to execute request to Faulty Server")
            .json::<FaultyServerResponse>()
            .await
            .expect("Failed to parse json from Faulty Server")
    }
}

#[cfg(test)]
mod should {
    use super::*;
    use httpmock::{Method, MockServer};

    async fn mock(mock_server: &MockServer, status: u16, response_json: &FaultyServerResponse) {
        mock_server
            .mock_async(|when, then| {
                when.method(Method::GET).header_exists("X-Run-Id");
                then.status(status)
                    .header("Content-Type", "application/json")
                    .json_body_obj(response_json);
            })
            .await;
    }

    #[actix_rt::test]
    async fn handle_200_ok_responses_correctly() {
        let mock_server = MockServer::start_async().await;
        let sender =
            ReqwestRequestSender::new(Client::new(), format!("http://{}", mock_server.address()));

        let expected_response = FaultyServerResponse::Ok { value: 50 };
        mock(&mock_server, 200, &expected_response).await;

        let actual_response = sender.send_request(RunId::new_v4()).await;
        assert_eq!(expected_response, actual_response)
    }

    #[actix_rt::test]
    async fn handle_500_internal_server_error_responses_correctly() {
        let mock_server = MockServer::start_async().await;
        let sender =
            ReqwestRequestSender::new(Client::new(), format!("http://{}", mock_server.address()));

        let expected_response = FaultyServerResponse::Err {
            error: "Internal server error".into(),
        };
        mock(&mock_server, 500, &expected_response).await;

        let actual_response = sender.send_request(RunId::new_v4()).await;
        assert_eq!(expected_response, actual_response);
    }

    #[actix_rt::test]
    async fn handle_504_timed_out_responses_correctly() {
        let mock_server = MockServer::start_async().await;
        let sender =
            ReqwestRequestSender::new(Client::new(), format!("http://{}", mock_server.address()));

        let expected_response = FaultyServerResponse::Err {
            error: "Timed out".into(),
        };
        mock(&mock_server, 504, &expected_response).await;

        let actual_response = sender.send_request(RunId::new_v4()).await;
        assert_eq!(expected_response, actual_response);
    }

    #[actix_rt::test]
    async fn handle_429_too_many_requests_responses_correctly() {
        let mock_server = MockServer::start_async().await;
        let sender =
            ReqwestRequestSender::new(Client::new(), format!("http://{}", mock_server.address()));

        let expected_response = FaultyServerResponse::Err {
            error: "Too many concurrent requests".into(),
        };
        mock(&mock_server, 429, &expected_response).await;

        let actual_response = sender.send_request(RunId::new_v4()).await;
        assert_eq!(expected_response, actual_response);
    }
}
