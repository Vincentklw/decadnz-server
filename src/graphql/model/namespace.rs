use crate::database;
use crate::graphql::model::object::Object;
use crate::graphql::request_context::RequestContext;
use juniper::{graphql_object, FieldResult};

pub struct Namespace {
    pub name: String,
}

#[graphql_object(context = RequestContext)]
impl Namespace {
    pub async fn namespaces(context: &RequestContext) -> FieldResult<Vec<Namespace>> {
        let namespaces = context.databases.object_database.get_all_namespaces().await?;
        Ok(namespaces.iter().map(Namespace::from).collect())
    }

    pub fn name(&self) -> &String { &self.name }

    pub async fn objects(&self, context: &RequestContext) -> FieldResult<Vec<Object>> {
        Object::by_namespace(self.name.clone(), context).await
    }
}

impl From<database::model::namespace::Namespace> for Namespace {
    fn from(value: database::model::namespace::Namespace) -> Self {
        Self {
            name: value.namespace,
        }
    }
}

impl From<&database::model::namespace::Namespace> for Namespace {
    fn from(value: &database::model::namespace::Namespace) -> Self {
        Self {
            name: value.namespace.clone(),
        }
    }
}