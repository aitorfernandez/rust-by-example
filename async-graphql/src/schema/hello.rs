use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct HelloQuery;

#[Object]
impl HelloQuery {
    async fn hello(&self, _ctx: &Context<'_>) -> Result<String> {
        Ok("Hello, world".to_string())
    }
}
