use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Schema {
    Null,
    Boolean,
    Number,
    String,
    Array(BTreeSet<Schema>),
    Object(BTreeMap<String, Schema>),
}
