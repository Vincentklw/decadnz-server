use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::Type, sqlx::FromRow, Debug, Clone)]
pub struct Object {
    pub object_id: Vec<u8>,
    pub namespace: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}