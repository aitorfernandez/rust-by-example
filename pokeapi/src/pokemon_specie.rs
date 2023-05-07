use crate::api_client::{ApiClient, ApiClientError};
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Shape {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EggGroup {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSpecie {
    pub name: String,
    pub shape: Shape,
    pub egg_groups: Vec<EggGroup>,
}

impl PokemonSpecie {
    pub async fn get(id_or_name: &str) -> Result<PokemonSpecie, PokemonError> {
        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{id_or_name}");
        let res = ApiClient::new().make_request(&url).await?;

        serde_json::from_value(res).map_err(PokemonError::InvalidJson)
    }
}
