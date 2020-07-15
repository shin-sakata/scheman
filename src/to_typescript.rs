use crate::schema::Schema;
use crate::schema::Schema as S;

use std::collections::{BTreeMap, BTreeSet};

use inflector::Inflector;

pub fn to_typescript(schema: &Schema) -> String {
    let prefix = "type Schema = ".to_string();
    prefix + &to_typescript_internal(schema)
}

fn to_typescript_internal(schema: &Schema) -> String {
    match schema {
        S::Null => "null\n".to_string(),
        S::Boolean => "boolean\n".to_string(),
        S::Number => "number\n".to_string(),
        S::String => "string\n".to_string(),
        S::Array(set) => array_to_lines(set),
        S::Object(obj) => format!("{{{}}};\n", obj_to_lines(obj.clone())),
    }
}

fn array_to_lines(set: &BTreeSet<Schema>) -> String {
    if set.len() != 1 {
        "any;\n".to_string()
    } else {
        let tmp = set.iter().collect::<Vec<_>>();
        let schema = tmp.first().unwrap();
        format!("Array<{}>;\n", to_typescript_internal(schema))
    }
}

fn obj_to_lines(object: BTreeMap<String, Schema>) -> String {
    object
        .iter()
        .map(|(key, val)| format!("{}: {}", key.to_camel_case(), to_typescript_internal(val)))
        .collect::<Vec<String>>()
        .join("")
}
