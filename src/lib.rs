use pgx::*;
use std::fs;

pub mod errors;
pub mod json_loader;

use crate::json_loader::ddl_provider::DDLProvider;
use crate::json_loader::dml_provider::DMLProvider;
use crate::json_loader::entities::{JsonStructure, TableMap};

pg_module_magic!();

fn load_json(path: &str) -> JsonStructure {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    return serde_json::from_str(&contents).unwrap();
}

fn generate_ddl_from_json(parsed_json: &JsonStructure) -> (TableMap, String) {
    let ddl_provider: DDLProvider = DDLProvider::new();
    return ddl_provider.generate_create_table(parsed_json.to_owned());
}

fn generate_dml_from_json(parsed_json: &JsonStructure) -> String {
    let dml_provider: DMLProvider = DMLProvider::new();
    return dml_provider.generate_table_inserts(parsed_json.to_owned());
}

#[pg_extern]
fn run_json(path: &str) {
    let parsed_json: JsonStructure = load_json(path);
    let (_attribute_map, ddl_sql) = generate_ddl_from_json(&parsed_json);

    Spi::run(ddl_sql.as_str());

    let dml_sql = generate_dml_from_json(&parsed_json);
    
    Spi::run(dml_sql.as_str());
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_load_dml_json() {
        let json_structure = crate::load_json("/home/hizagi/projects/jgres/test.json");
        assert_eq!("INSERT INTO products (id,name,quantity,value) VALUES (12,\'product1\',11,1200);", crate::generate_dml_from_json(&json_structure))
    }

    #[pg_test]
    fn test_load_ddl_json() {}

    #[pg_test]
    fn test_run_json() {
        crate::run_json("/home/hizagi/projects/jgres/test.json")
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
