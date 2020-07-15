use std::collections::HashMap;

type Object = HashMap<String, Schema>;
type Array = Vec<Schema>;

pub enum Schema {
    Null,
    Boolean,
    Number,
    String,
    Array(Array),
    Object(Object),
}
