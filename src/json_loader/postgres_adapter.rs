use crate::json_loader::entities::{JsonStructure, Table, Attribute};
use crate::errors::invalid_table_definition::InvalidTableDefinition;

pub struct PostgresAdapter {}

impl PostgresAdapter {
    pub fn json_structure_to_sql(json_structure: JsonStructure) -> String {
        let mut query = "".to_owned();
        let tables = json_structure.tables;

        for table in tables.iter() {
            let table_query = PostgresAdapter::table_json_to_sql(table).unwrap_or_else(|error| {
                panic!("Problem: {:?}", error)
            });
            
            query.push_str(&table_query);
        }

        return query;
    }
    
    pub fn table_json_to_sql(table: &Table) -> Result<String, InvalidTableDefinition> {
        if table.name.len() < 1 {
            return Err(InvalidTableDefinition{
                tablename:  String::from(table.name.as_str()),
                reason: String::from("No table name")
            });
        }

        let sql_str = format!("CREATE TABLE {} (id);", table.name); 

        return Ok(String::from(sql_str));
    }

    pub fn attribute_json_to_sql(_attributes: Vec<Attribute>) -> String {
        return String::from("123");
    }
}