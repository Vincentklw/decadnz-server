pub(crate) mod database;
pub(crate) mod graphql;
pub(crate) mod error;

use crate::database::attribute_database::{AttributeDatabase, AttributeDatabaseTrait};
use crate::database::object_database::{ObjectDatabase, ObjectDatabaseTrait};
use crate::graphql::schema::SchemaDependencies;
use crate::graphql::webserver::Webserver;
use dotenv::dotenv;
use log::info;
use sqlx::mysql::MySqlPoolOptions;
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

    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:pw@localhost:3306/decadnz").await?;

    let object_db: Arc<Box<dyn ObjectDatabaseTrait>> = Arc::new(Box::new(ObjectDatabase::init(pool.clone()).await?));
    let attribute_db: Arc<Box<dyn AttributeDatabaseTrait>> = Arc::new(Box::new(AttributeDatabase::init(pool.clone()).await?));

    let graphql_webserver = Webserver {};

    let graphql_schema_dependencies = SchemaDependencies {
        object_database: object_db.clone(),
    };

    let gql_thread = tokio::spawn(graphql_webserver.serve(6111, graphql_schema_dependencies).await?);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl+C received, shutting down gracefully...");
            gql_thread.abort();
        }
    }

    let _ = tokio::join!(gql_thread);

    Ok(())
}
