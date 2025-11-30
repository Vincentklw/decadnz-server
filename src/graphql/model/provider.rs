use crate::graphql::request_context::RequestContext;
use crate::{database, util};
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

pub struct Provider {
    provider_id: Uuid,
    provider_name: String,
    api_endpoint: String,
    api_endpoint_access_token: String,
}

impl Provider {
    pub async fn create(context: &RequestContext, name: String, api_endpoint: String, api_endpoint_access_token: String) -> FieldResult<bool> {
        context.databases.provider_database.create(name, api_endpoint, api_endpoint_access_token).await?;
        Ok(true)
    }

    pub async fn by_id(id: Uuid, context: &RequestContext) -> FieldResult<Option<Provider>> {
        let res = context.databases.provider_database.get_by_id(id).await?;

        Ok(Some(Provider::from(res.unwrap())))
    }
}

#[graphql_object(context = RequestContext)]
impl Provider {
    pub fn id(&self) -> &Uuid { &self.provider_id }

    pub fn name(&self) -> &String { &self.provider_name }

    pub fn api_endpoint(&self) -> &String { &self.api_endpoint }
}

impl From<database::model::provider::Provider> for Provider {
    fn from(value: database::model::provider::Provider) -> Self {
        Self {
            // TODO can throw
            provider_id: util::transform::vector_to_uuid(value.provider_id).unwrap(),
            provider_name: value.provider_name,
            api_endpoint: value.api_endpoint,
            api_endpoint_access_token: value.api_endpoint_access_token,
        }
    }
}