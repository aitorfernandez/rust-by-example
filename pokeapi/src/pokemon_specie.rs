use crate::{api_client::ApiClient, error::PokemonError};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
    pub version: Version,
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
    pub egg_groups: Vec<EggGroup>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub name: String,
    pub shape: Shape,
}

impl PokemonSpecie {
    pub async fn get(id_or_name: &str) -> Result<PokemonSpecie, PokemonError> {
        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{id_or_name}");
        let res = ApiClient::new().make_request(Method::GET, &url).await?;

        serde_json::from_value(res).map_err(PokemonError::InvalidJson)
    }

    pub fn description(&self) -> Option<String> {
        self.flavor_text_entries
            .iter()
            .find(|f| f.language.name == "en")
            .map(|v| v.flavor_text.to_string())
    }
}
