use crate::database::database::SqliteDatabase;
use crate::error::Error;

use sqlx::{query_file, MySqlPool};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct ObjectDatabase {
    pub pool: MySqlPool,
}

impl ObjectDatabase {
    pub async fn init(pool: MySqlPool) -> Result<Self, Error> {
        let db = Self {
            pool,
        };
        db.create_table().await?;
        Ok(db)
    }
}

pub trait ObjectDatabaseTrait: SqliteDatabase + Sync + Send {
    fn create(&self, namespace: String, name: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;
}

impl SqliteDatabase for ObjectDatabase {
    fn create_table(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/object/migration.sql").execute(&self.pool).await?;
            Ok(())
        })
    }
}

impl ObjectDatabaseTrait for ObjectDatabase {
    fn create(&self, namespace: String, name: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/object/insert.sql", namespace, name).execute(&self.pool).await?;
            Ok(())
        })
    }
}