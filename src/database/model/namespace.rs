use sqlx::FromRow;

#[derive(FromRow)]
pub struct Namespace {
    pub namespace: String,
}
