use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Error {
    error: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self {
            error: value.to_string()
        }
    }
}
