use crate::database::database::SqliteDatabase;
use crate::database::model::attribute::Attribute;
use crate::error::Error;
use sqlx::types::Uuid;
use sqlx::{query_file, query_file_as, MySqlPool};
use std::pin::Pin;

#[derive(Clone)]
pub struct AttributeDatabase {
    pub pool: MySqlPool,
}

impl AttributeDatabase {
    pub async fn init(pool: MySqlPool) -> Result<Self, Error> {
        let db = Self {
            pool,
        };
        db.create_table().await?;
        Ok(db)
    }
}

pub trait AttributeDatabaseTrait: SqliteDatabase + Sync + Send {
    fn create(&self, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: Option<String>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;

    fn get_all_by_object_id(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Vec<Attribute>, Error>> + Send + '_>>;
}

impl SqliteDatabase for AttributeDatabase {
    fn create_table(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/attribute/migration.sql").execute(&self.pool).await?;
            Ok(())
        })
    }
}

impl AttributeDatabaseTrait for AttributeDatabase {
    fn create(&self, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: Option<String>) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/attribute/insert.sql", object_id, name, description, r#type, default_value).execute(&self.pool).await?;
            Ok(())
        })
    }

    fn get_all_by_object_id(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Vec<Attribute>, Error>> + Send + '_>> {
        Box::pin(async move {
            Ok(query_file_as!(Attribute, "sql/attribute/get_all_by_object_id.sql", object_id).fetch_all(&self.pool).await?)
        })
    }
}