#[actix_web::main]
async fn main() -> std::io::Result<()> {
    async_graphql_example::http_server::start().await
}
