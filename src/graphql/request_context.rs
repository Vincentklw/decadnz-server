use juniper::Context;

#[derive(Clone, Default)]
pub struct RequestContext {
    pub auth_token: Option<String>,
}

impl Context for RequestContext {}