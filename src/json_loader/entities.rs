use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub data_type: Option<String>,
    pub is_primary: Option<bool>,
    pub not_null: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
   pub name: String,
   pub attributes: Vec<Attribute>,
}


#[derive(Serialize, Deserialize)]
pub struct JsonStructure {
    pub tables: Vec<Table>,
}