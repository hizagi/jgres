use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub data_type: Option<String>,
    pub native_type: Option<String>,
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