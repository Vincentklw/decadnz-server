use std::str::FromStr;
use uuid::{Error, Uuid};

pub fn vector_to_uuid(uuid: Vec<u8>) -> Result<Uuid, Error> {
    let str = String::from_utf8(uuid).unwrap();
    Uuid::from_str(&*str)
}