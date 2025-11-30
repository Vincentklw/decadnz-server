use crate::database::attribute_database::AttributeDatabaseTrait;
use crate::database::object_database::ObjectDatabaseTrait;
use crate::database::provider_database::ProviderDatabaseTrait;
use juniper::Context;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContextDatabase {
    pub object_database: Arc<Box<dyn ObjectDatabaseTrait>>,
    pub attribute_database: Arc<Box<dyn AttributeDatabaseTrait>>,
    pub provider_database: Arc<Box<dyn ProviderDatabaseTrait>>,
}

#[derive(Clone)]
pub struct RequestContext {
    pub auth_token: Option<String>,
    pub databases: RequestContextDatabase,
}

impl Context for RequestContext {}