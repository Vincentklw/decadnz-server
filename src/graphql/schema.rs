use crate::graphql::request_context::RequestContext;

use crate::graphql::mutation::mutation::Mutation;
use crate::graphql::query::query::Query;
use juniper::{EmptySubscription, RootNode};

pub type Schema = RootNode<Query, Mutation, EmptySubscription<RequestContext>>;

pub fn schema() -> Schema {
    let query = Query {};
    let mutation = Mutation {};

    Schema::new(query, mutation, EmptySubscription::<RequestContext>::new())
}