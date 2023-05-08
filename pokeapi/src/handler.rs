use crate::{error::PokemonError, pokemon_specie::PokemonSpecie};
// use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

// #[get("/pokemon-species/{id_or_name}")]
// async fn pokemon_species(path: web::Path<(String,)>) -> impl Responder {
//     let (id_or_name,) = path.into_inner();
//     let res = PokemonSpecie::get(&id_or_name).await;

//     HttpResponse::Ok().json(res)
// }

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    pub description: String,
    pub name: String,
}

impl From<PokemonSpecie> for Pokemon {
    fn from(ps: PokemonSpecie) -> Self {
        let description = ps.description().unwrap_or("".to_string());
        let PokemonSpecie { name, .. } = ps;

        Self { name, description }
    }
}

#[get("/pokemon-species/{id_or_name}")]
async fn pokemon_species(path: web::Path<(String,)>) -> Result<HttpResponse, PokemonError> {
    let (id_or_name,) = path.into_inner();
    let pokemon_specie = PokemonSpecie::get(&id_or_name).await?;

    Ok(HttpResponse::Ok().json(Pokemon::from(pokemon_specie)))
}

pub fn configure_service(conf: &mut web::ServiceConfig) {
    conf.service(pokemon_species);
}
