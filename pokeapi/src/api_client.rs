use crate::error::ApiClientError;
use reqwest::{Client, Method, StatusCode};
use tracing::instrument;

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    #[instrument(level = "info")]
    pub async fn make_request(
        &self,
        method: Method,
        url: &str,
    ) -> Result<serde_json::Value, ApiClientError> {
        let res = self.client.request(method, url).send().await?;

        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                let json = serde_json::from_str(&body)?;
                Ok(json)
            }
            StatusCode::NOT_FOUND => Err(ApiClientError::NotFound),
            _ => Err(ApiClientError::UnexpectedStatusCode(res.status())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn make_request_ok_test() {
        // Arrange
        let response = r#"{
            "id": 123,
            "name": "Test User"
        }"#;
        let mut server = mockito::Server::new();
        let _ = server
            .mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();
        let url = format!("{}/test", server.url());

        // Act
        let result = ApiClient::new().make_request(&url).await;

        // Assert
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn make_request_error() {
        // Test cases
        let test_cases = vec![
            (
                404,
                "Not found",
                format!("{:?}", Some(ApiClientError::NotFound)),
            ),
            (
                500,
                "Internal server error",
                format!(
                    "{:?}",
                    Some(ApiClientError::UnexpectedStatusCode(
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                ),
            ),
        ];

        for (status, body, expected_result) in test_cases {
            // Arrange
            let mut server = mockito::Server::new();
            let _ = server
                .mock("GET", "/test")
                .with_status(status)
                .with_header("content-type", "application/json")
                .with_body(body)
                .create();
            let url = format!("{}{}", server.url(), "/test");

            // Act
            let result = ApiClient::new().make_request(&url).await;

            // Assert
            assert_eq!(format!("{:?}", result.err()), expected_result);
        }
    }
}
