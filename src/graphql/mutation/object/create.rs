use crate::database::object_database::ObjectDatabaseTrait;
use juniper::FieldResult;
use std::sync::Arc;

pub struct Create {
    pub object_database: Arc<Box<dyn ObjectDatabaseTrait>>,
}

impl Create {
    pub async fn create(&self, namespace: String, name: String) -> FieldResult<String> {
        self.object_database.create(namespace, name).await?;
        Ok("ok".to_string())
    }
}