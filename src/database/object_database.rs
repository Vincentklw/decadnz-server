use crate::database::database::SqliteDatabase;
use crate::error::Error;

use crate::database::model::namespace::Namespace;
use crate::database::model::object::Object;
use sqlx::types::Uuid;
use sqlx::{query_file, query_file_as, MySqlPool};
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
    fn get_by_id(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Option<Object>, Error>> + Send + '_>>;
    fn get_by_namespace_and_name(&self, namespace: String, name: String) -> Pin<Box<dyn Future<Output=Result<Option<Object>, Error>> + Send + '_>>;
    fn get_by_namespace(&self, namespace: String) -> Pin<Box<dyn Future<Output=Result<Vec<Object>, Error>> + Send + '_>>;

    fn get_all_namespaces(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Namespace>, Error>> + Send + '_>>;

    fn delete(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;
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
        // TODO input validation
        Box::pin(async move {
            query_file!("sql/object/insert.sql", namespace.to_lowercase(), name.to_lowercase()).execute(&self.pool).await?;
            Ok(())
        })
    }

    fn get_by_id(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Option<Object>, Error>> + Send + '_>> {
        // TODO input validation
        Box::pin(async move {
            let res = query_file_as!(Object, "sql/object/get_by_id.sql", object_id).fetch_optional(&self.pool).await?;
            Ok(res)
        })
    }

    fn get_by_namespace_and_name(&self, namespace: String, name: String) -> Pin<Box<dyn Future<Output=Result<Option<Object>, Error>> + Send + '_>> {
        // TODO input validation
        Box::pin(async move {
            let res = query_file_as!(Object, "sql/object/get_by_namespace_and_name.sql", namespace.to_lowercase(), name.to_lowercase()).fetch_optional(&self.pool).await?;
            Ok(res)
        })
    }

    fn get_by_namespace(&self, namespace: String) -> Pin<Box<dyn Future<Output=Result<Vec<Object>, Error>> + Send + '_>> {
        // TODO input validation
        Box::pin(async move {
            let res = query_file_as!(Object, "sql/object/get_by_namespace.sql", namespace).fetch_all(&self.pool).await?;
            Ok(res)
        })
    }

    fn get_all_namespaces(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Namespace>, Error>> + Send + '_>> {
        // TODO input validation
        Box::pin(async move {
            let res = query_file_as!(Namespace, "sql/object/get_all_namespaces.sql").fetch_all(&self.pool).await?;

            Ok(res)
        })
    }

    fn delete(&self, object_id: Uuid) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        // TODO input validation
        Box::pin(async move {
            let res = query_file!("sql/object/delete_by_id.sql", object_id).execute(&self.pool).await?;
            Ok(())
        })
    }
}