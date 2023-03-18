use async_graphql::MergedObject;

mod hello;

pub use hello::HelloQuery;

#[derive(MergedObject, Default)]
pub struct Query(HelloQuery);
