mod api_client;
mod handler;
mod pokemon_specie;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(handler::configure_service))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
