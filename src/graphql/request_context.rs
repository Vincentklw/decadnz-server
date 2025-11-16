use crate::database::object_database::ObjectDatabaseTrait;
use juniper::Context;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContextDatabase {
    pub object_database: Arc<Box<dyn ObjectDatabaseTrait>>,
}

#[derive(Clone)]
pub struct RequestContext {
    pub auth_token: Option<String>,
    pub databases: RequestContextDatabase,
}

impl Context for RequestContext {}