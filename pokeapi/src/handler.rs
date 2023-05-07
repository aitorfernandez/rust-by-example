use crate::pokemon_specie::{PokemonError, PokemonSpecie};
// use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{get, web, HttpResponse};

// #[get("/pokemon-species/{id_or_name}")]
// async fn pokemon_species(path: web::Path<(String,)>) -> impl Responder {
//     let (id_or_name,) = path.into_inner();
//     let res = PokemonSpecie::get(&id_or_name).await;

//     HttpResponse::Ok().json(res)
// }

#[get("/pokemon-species/{id_or_name}")]
async fn pokemon_species(path: web::Path<(String,)>) -> Result<HttpResponse, PokemonError> {
    let (id_or_name,) = path.into_inner();
    let pokemon = PokemonSpecie::get(&id_or_name).await?;

    Ok(HttpResponse::Ok().json(pokemon))
}

pub fn configure_service(conf: &mut web::ServiceConfig) {
    conf.service(pokemon_species);
}
