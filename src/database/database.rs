use crate::error::Error;

use std::pin::Pin;

pub trait SqliteDatabase: Sync + Send {
    fn create_table(&self) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send + '_>>;
}