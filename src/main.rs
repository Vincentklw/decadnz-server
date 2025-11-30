pub(crate) mod database;
pub(crate) mod graphql;
pub(crate) mod error;
pub(crate) mod util;

use crate::database::attribute_database::{AttributeDatabase, AttributeDatabaseTrait};
use crate::database::object_database::{ObjectDatabase, ObjectDatabaseTrait};
use crate::database::provider_database::{ProviderDatabase, ProviderDatabaseTrait};
use crate::graphql::webserver::{ServerDependencies, Webserver};
use dotenv::dotenv;
use log::info;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::try_init()?;

    info!(r"");
    info!(r"     __                __       ");
    info!(r" ___/ /__ _______ ____/ /__  ___");
    info!(r"/ _  / -_) __/ _ `/ _  / _ \/_ /");
    info!(r"\_,_/\__/\__/\_,_/\_,_/_//_//__/");
    info!(r"");

    let app_version = env!("CARGO_PKG_VERSION");
    info!("Starting in version {}", app_version);
    info!(r"");

    let database_hostname = env::var("DATABASE_HOSTNAME")?;
    let database_port = env::var("DATABASE_PORT")?.parse::<i32>()?;
    let database_username = env::var("DATABASE_USERNAME")?;
    let database_password = env::var("DATABASE_PASSWORD")?;
    let database_database = env::var("DATABASE_DATABASE")?;

    let database_connection_string = format!("mysql://{}:{}@{}:{}/{}", database_username, database_password, database_hostname, database_port, database_database);

    let pool = MySqlPoolOptions::new()
        .connect(&database_connection_string).await?;

    let object_db: Arc<Box<dyn ObjectDatabaseTrait>> = Arc::new(Box::new(ObjectDatabase::init(pool.clone()).await?));
    let attribute_db: Arc<Box<dyn AttributeDatabaseTrait>> = Arc::new(Box::new(AttributeDatabase::init(pool.clone()).await?));
    let provider_db: Arc<Box<dyn ProviderDatabaseTrait>> = Arc::new(Box::new(ProviderDatabase::init(pool.clone()).await?));

    let graphql_webserver = Webserver {};

    let graphql_schema_dependencies = ServerDependencies {
        object_database: object_db.clone(),
        attribute_database: attribute_db.clone(),
        provider_database: provider_db.clone(),
    };

    let app_port = env::var("APP_PORT")?.parse::<i32>()?;

    let gql_thread = tokio::spawn(graphql_webserver.serve(app_port, graphql_schema_dependencies).await?);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl+C received, shutting down gracefully...");
            gql_thread.abort();
        }
    }

    let _ = tokio::join!(gql_thread);

    Ok(())
}
