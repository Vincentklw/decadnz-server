use crate::graphql::request_context::RequestContext;
use std::sync::Arc;

use crate::database::object_database::ObjectDatabaseTrait;
use crate::graphql::mutation::mutation::Mutation;
use crate::graphql::query::query::Query;
use juniper::{EmptySubscription, RootNode};

pub type Schema = RootNode<Query, Mutation, EmptySubscription<RequestContext>>;

#[derive(Clone)]
pub struct SchemaDependencies {
    pub object_database: Arc<Box<dyn ObjectDatabaseTrait>>,
}

pub fn schema() -> Schema {
    let query = Query {};
    let mutation = Mutation {};

    Schema::new(query, mutation, EmptySubscription::<RequestContext>::new())
}