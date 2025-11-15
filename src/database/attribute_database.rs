use crate::database::database::SqliteDatabase;
use crate::error::Error;
use sqlx::types::Uuid;
use sqlx::{query_file, MySqlPool};
use std::pin::Pin;

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
    fn create(&self, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;
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
    fn create(&self, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/attribute/insert.sql", object_id, name, description, r#type, default_value).execute(&self.pool).await?;
            Ok(())
        })
    }
}