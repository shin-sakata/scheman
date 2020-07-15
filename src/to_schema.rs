use crate::schema::Schema as S;
use crate::schema::Schema;

use serde_json::Value;
use serde_json::Value as V;

pub trait ToSchema {
    fn to_schema(&self) -> Schema;
}

impl ToSchema for Value {
    fn to_schema(&self) -> Schema {
        match self {
            V::Null => S::Null,
            V::Bool(_) => S::Boolean,
            V::Number(_) => S::Number,
            V::String(_) => S::String,
            V::Array(arr) => S::Array(arr.into_iter().map(|item| item.to_schema()).collect()),
            V::Object(obj) => S::Object(
                obj.iter()
                    .map(|(key, val)| (key.clone(), val.to_schema()))
                    .collect(),
            ),
        }
    }
}
