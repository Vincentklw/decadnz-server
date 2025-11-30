use crate::database::database::SqliteDatabase;
use crate::database::model::attribute::Attribute;
use crate::database::model::provider::Provider;
use crate::error::Error;
use sqlx::{query_file, query_file_as, MySqlPool};
use std::pin::Pin;
use uuid::Uuid;

pub struct ProviderDatabase {
    pub pool: MySqlPool,
}

impl ProviderDatabase {
    pub async fn init(pool: MySqlPool) -> Result<Self, Error> {
        let db = Self {
            pool,
        };
        db.create_table().await?;
        Ok(db)
    }
}

pub trait ProviderDatabaseTrait: SqliteDatabase + Sync + Send {
    fn create(&self, provider_name: String, api_endpoint: String, api_endpoint_access_token: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;

    fn get_all(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Provider>, Error>> + Send + '_>>;

    fn get_by_id(&self, provider_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Option<Provider>, Error>> + Send + '_>>;
}

impl SqliteDatabase for ProviderDatabase {
    fn create_table(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/provider/migration.sql").execute(&self.pool).await?;
            Ok(())
        })
    }
}

impl ProviderDatabaseTrait for ProviderDatabase {
    fn create(&self, provider_name: String, api_endpoint: String, api_endpoint_access_token: String) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>> {
        Box::pin(async move {
            query_file!("sql/provider/insert.sql", provider_name, api_endpoint, api_endpoint_access_token).execute(&self.pool).await?;
            Ok(())
        })
    }

    fn get_all(&self) -> Pin<Box<dyn Future<Output=Result<Vec<Provider>, Error>> + Send + '_>> {
        Box::pin(async move {
            Ok(query_file_as!(Provider, "sql/provider/get_all.sql").fetch_all(&self.pool).await?)
        })
    }

    fn get_by_id(&self, provider_id: Uuid) -> Pin<Box<dyn Future<Output=Result<Option<Provider>, Error>> + Send + '_>> {
        Box::pin(async move {
            Ok(query_file_as!(Provider, "sql/provider/get_by_id.sql", provider_id).fetch_optional(&self.pool).await?)
        })
    }
}