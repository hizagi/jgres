use std::fs;
use crate::json_loader::entities::JsonStructure;
use crate::json_loader::postgres_adapter::PostgresAdapter;

#[derive(Default)]
pub struct JsonProvider {
    filepath: String     
}

impl JsonProvider {
    pub fn new(path: &str) -> JsonProvider {
        JsonProvider{
            filepath: String::from(path),
        }
    }

    pub fn load_json(&self) -> String {
        let contents = fs::read_to_string(self.filepath.as_str())
        .expect("Something went wrong reading the file");

        let parsed: JsonStructure = serde_json::from_str(&contents).unwrap();

        return PostgresAdapter::json_structure_to_sql(parsed)
    }
}