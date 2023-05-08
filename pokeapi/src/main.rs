mod api_client;
mod error;
mod fun_translation;
mod handler;
mod pokemon_specie;

use actix_web::{App, HttpServer};
use tracing::{info, Level};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("global subscriber hasn't been set");

    info!("Starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .configure(handler::configure_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
