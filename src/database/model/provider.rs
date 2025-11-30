use sqlx::FromRow;

#[derive(FromRow)]
pub struct Provider {
    pub provider_id: Vec<u8>,
    pub provider_name: String,
    pub api_endpoint: String,
    pub api_endpoint_access_token: String,
}



