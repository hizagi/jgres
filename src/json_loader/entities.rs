use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub name: String,
    pub data_type: Option<String>,
    pub is_primary: Option<bool>,
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tablename: String,
    pub data: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonStructure {
    pub tables: Vec<Table>,
    pub dataset: Vec<Data>,
}

pub type TableMap = BTreeMap<String, Vec<String>>;
