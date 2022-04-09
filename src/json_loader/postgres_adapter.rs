use crate::json_loader::entities::{JsonStructure};
use crate::json_loader::ddl_provider::DDLProvider;

pub struct PostgresAdapter {}

impl PostgresAdapter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn json_structure_to_sql(&self, json_structure: JsonStructure) -> String {
        let ddl_provider:DDLProvider = DDLProvider::new();

        return ddl_provider.generate_create_table(json_structure)
    }
}