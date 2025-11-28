use chrono::NaiveDateTime;

#[derive(sqlx::Type, sqlx::FromRow, Debug, Clone)]
pub struct Attribute {
    pub attribute_id: Vec<u8>,
    pub object_id: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
    pub default_value: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}