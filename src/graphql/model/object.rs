use crate::graphql::model::attribute::Attribute;
use crate::graphql::request_context::RequestContext;
use crate::{database, util};
use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldResult};
use std::str::FromStr;
use uuid::Uuid;

pub struct Object {
    object_id: Uuid,
    namespace: String,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<database::model::object::Object> for Object {
    fn from(value: database::model::object::Object) -> Self {
        Self {
            object_id: util::transform::vector_to_uuid(value.object_id).unwrap(),
            namespace: value.namespace,
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Object {
    pub async fn create(context: &RequestContext, namespace: String, name: String) -> FieldResult<bool> {
        let _ = context.databases.object_database.create(namespace, name).await?;
        Ok(true)
    }
    pub async fn delete(context: &RequestContext, object_id: Uuid) -> FieldResult<bool> {
        let _ = context.databases.object_database.delete(object_id).await?;
        Ok(true)
    }


    pub async fn by_id(id: Uuid, context: &RequestContext) -> FieldResult<Option<Object>> {
        let res = context.databases.object_database.get_by_id(id).await?;

        Ok(Some(Object::from(res.unwrap())))
    }

    pub async fn by_namespace(namespace: String, context: &RequestContext) -> FieldResult<Vec<Object>> {
        let res = context.databases.object_database.get_by_namespace(namespace.clone()).await?;
        let mut arr = Vec::new();
        res.iter().for_each(|p| arr.push(Object::from(p.clone())));
        Ok(arr)
    }
}

#[graphql_object(context = RequestContext)]
impl Object {
    pub fn object_id(&self) -> &Uuid { &self.object_id }

    pub fn namespace(&self) -> &String { &self.namespace }

    pub fn name(&self) -> &String { &self.name }

    pub fn created_at(&self) -> &NaiveDateTime { &self.created_at }

    pub fn updated_at(&self) -> &NaiveDateTime { &self.updated_at }

    pub async fn attributes(&self, context: &RequestContext) -> FieldResult<Vec<Attribute>> {
        Attribute::by_object_id(context, self.object_id).await
    }
}