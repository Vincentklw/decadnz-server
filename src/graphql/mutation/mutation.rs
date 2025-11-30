use crate::graphql::request_context::RequestContext;

use crate::graphql::model::attribute::Attribute;
use crate::graphql::model::object::Object;
use crate::graphql::model::provider::Provider;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct Mutation {}

#[graphql_object(context = RequestContext)]
impl Mutation {
    async fn create_object(&self, context: &RequestContext, namespace: String, name: String) -> FieldResult<bool> {
        Object::create(context, namespace, name).await
    }

    async fn delete_object(&self, context: &RequestContext, object_id: Uuid) -> FieldResult<bool> {
        Object::delete(context, object_id).await
    }

    pub async fn create_attribute(&self, context: &RequestContext, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: Option<String>) -> FieldResult<bool> {
        Attribute::create(context, object_id, name, description, r#type, default_value).await
    }

    pub async fn create_data_provider(&self, context: &RequestContext, name: String, api_endpoint: String, api_endpoint_access_token: String) -> FieldResult<bool> {
        Provider::create(context, name, api_endpoint, api_endpoint_access_token).await
    }
}