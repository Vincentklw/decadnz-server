use uuid::Uuid;

pub struct Provider {
    provider_id: Uuid,
    provider_identifier: String,
    api_endpoint: String,
    api_endpoint_access_token: String,
}