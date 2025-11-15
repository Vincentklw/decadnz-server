use crate::graphql::request_context::RequestContext;

use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = RequestContext)]
impl Mutation {
    async fn api_version(&self) -> FieldResult<String> {
        let app_version = env!("CARGO_PKG_VERSION");
        Ok(app_version.to_string())
    }
}