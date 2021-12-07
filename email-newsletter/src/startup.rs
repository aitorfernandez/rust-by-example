use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let pool = web::Data::new(pool);
    // HttpServer handles all transport level concerns.
    let server = HttpServer::new(move || {
        // App is where all application logic lives, routing, middlewares, request...
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
