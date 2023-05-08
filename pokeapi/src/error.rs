use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("Resource not found")]
    NotFound,
    #[error("Bad status code: {0}")]
    UnexpectedStatusCode(StatusCode),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum PokemonError {
    #[error("Serialization/deserialization error: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("ApiClient error: {0}")]
    ApiClient(#[from] ApiClientError),
}

impl ResponseError for PokemonError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PokemonError::InvalidJson(err) => HttpResponse::BadRequest().body(err.to_string()),
            PokemonError::ApiClient(ApiClientError::NotFound) => {
                HttpResponse::NotFound().body("The requested resource was not found")
            }
            _ => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}
