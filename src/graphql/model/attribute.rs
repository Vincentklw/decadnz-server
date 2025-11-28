use crate::error::Error;
use crate::graphql::model::object::Object;
use crate::graphql::request_context::RequestContext;
use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldResult};
use std::str::FromStr;
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use uuid::Uuid;

#[derive(Debug, EnumIter, AsRefStr, PartialEq)]
enum AttributeType {
    Bool,
    String,
    Long,
    Set,
    Record,
    Datetime,
    Decimal,
    Duration,
    IpAddr,
}

impl FromStr for AttributeType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = AttributeType::iter().find(|a| {
            s.to_lowercase() == a.as_ref().to_lowercase()
        });
        if let Some(item) = res {
            return Ok(item);
        }
        Err(Error { error: "did not find enum".to_string() })
    }
}

enum AttributeValue {
    Bool(bool),
    String(String),
    Long(i64),
    Set(Vec<AttributeType>),
    Record(Object),
    Datetime(NaiveDateTime),
    Decimal(f64),
    Duration(i64),
    IpAddr(String),
}

pub struct Attribute {
    attribute_id: Uuid,
    name: String,
    description: Option<String>,
    r#type: AttributeType,
    default_value: AttributeValue,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[graphql_object(context = RequestContext)]
impl Attribute {
    pub async fn create(context: &RequestContext, object_id: Uuid, name: String, description: Option<String>, r#type: String, default_value: Option<String>) -> FieldResult<bool> {
        context.databases.attribute_database.create(object_id, name, description, r#type, default_value).await?;
        Ok(true)
    }

    pub async fn by_object_id(context: &RequestContext, object_id: Uuid) -> FieldResult<Vec<Self>> {
        let res = context.databases.attribute_database.get_all_by_object_id(object_id).await?;
        let mut arr = Vec::new();
        res.iter().for_each(|p| arr.push(Attribute::from(p.clone())));
        Ok(arr)
    }

    pub fn attribute_id(&self) -> &Uuid { &self.attribute_id }
    pub fn name(&self) -> &String { &self.name }

    pub fn description(&self) -> &Option<String> { &self.description }

    pub fn r#type(&self) -> String { self.r#type.as_ref().to_string().to_lowercase() }
}

impl From<crate::database::model::attribute::Attribute> for Attribute {
    fn from(value: crate::database::model::attribute::Attribute) -> Self {
        let str = String::from_utf8(value.attribute_id).unwrap();
        Self {
            attribute_id: Uuid::from_str(&*str).unwrap(),
            name: value.name,
            description: value.description,
            r#type: AttributeType::from_str(value.r#type.as_str()).unwrap(),
            default_value: AttributeValue::Bool(false),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graphql::model::attribute::AttributeType;
    use std::str::FromStr;
    #[test]
    pub fn from_string() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(AttributeType::from_str("bool")?, AttributeType::Bool);
        assert_eq!(AttributeType::from_str("StrIng")?, AttributeType::String);
        assert_eq!(AttributeType::from_str("LONG")?, AttributeType::Long);
        assert_eq!(AttributeType::from_str("set")?, AttributeType::Set);
        assert_eq!(AttributeType::from_str("Record")?, AttributeType::Record);
        assert_eq!(AttributeType::from_str("Datetime")?, AttributeType::Datetime);
        assert_eq!(AttributeType::from_str("Decimal")?, AttributeType::Decimal);
        assert_eq!(AttributeType::from_str("Duration")?, AttributeType::Duration);
        assert_eq!(AttributeType::from_str("IpAddr")?, AttributeType::IpAddr);
        Ok(())
    }
}