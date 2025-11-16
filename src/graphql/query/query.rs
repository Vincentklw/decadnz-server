use crate::graphql::model::namespace::Namespace;
use crate::graphql::model::object::Object;
use crate::graphql::request_context::RequestContext;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct Query;

#[graphql_object(context = RequestContext)]
impl Query {
    async fn api_version(&self) -> FieldResult<String> {
        let app_version = env!("CARGO_PKG_VERSION");
        Ok(app_version.to_string())
    }

    async fn object(&self, context: &RequestContext, object_id: Uuid) -> FieldResult<Option<Object>> {
        Object::by_id(object_id, context).await
    }

    async fn namespace(&self, context: &RequestContext, namespace: String) -> FieldResult<Namespace> {
        Ok(Namespace {
            name: namespace,
        })
    }

    async fn namespaces(&self, context: &RequestContext) -> FieldResult<Vec<Namespace>> {
        Namespace::namespaces(context).await
    }
}