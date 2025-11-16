use crate::graphql::request_context::RequestContext;

use crate::graphql::model::object::Object;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct Mutation {}

#[graphql_object(context = RequestContext)]
impl Mutation {
    async fn api_version(&self) -> FieldResult<String> {
        let app_version = env!("CARGO_PKG_VERSION");
        Ok(app_version.to_string())
    }

    async fn create_object(&self, context: &RequestContext, namespace: String, name: String) -> FieldResult<bool> {
        Ok(Object::create(context, namespace, name).await?)
    }

    async fn delete_object(&self, context: &RequestContext, object_id: Uuid) -> FieldResult<bool> {
        Ok(Object::delete(context, object_id).await?)
    }
}