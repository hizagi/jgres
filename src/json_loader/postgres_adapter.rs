use crate::json_loader::entities::{JsonStructure, Table, Attribute};
use crate::errors::invalid_table_definition::InvalidTableDefinition;

pub struct PostgresAdapter {}

impl PostgresAdapter {
    pub fn json_structure_to_sql(json_structure: JsonStructure) -> String {
        let mut query = "".to_owned();
        let tables = json_structure.tables;

        for table in tables.iter() {
            let table_query = match query.push_str(table) {
                Ok(table_query) => table_query,
                Err(e) => ""
            };   
            
            query.push_str(&table_query);
        }

        return query;
    }
    
    pub fn table_json_to_sql(table: Table) -> Result<String, InvalidTableDefinition> {
        if table.name.len() < 1 {
            return Err(InvalidTableDefinition{
                tablename: table.name,
                reason: "No table name"
            });
        }

        let sql_str = format!("CREATE TABLE {} (id);", table.name); 

        Ok(String::from(sql_str));
    }

    pub fn attribute_json_to_sql(_attributes: Vec<Attribute>) -> String {
        return String::from("123");
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_json_structure_to_sql() {
        assert_eq!("12345", PostgresAdapter::json_structure_to_sql(JsonStructure{
            tables: vec![
                Table{
                    name: String::from("12345"),
                    attributes: vec![]
                }
            ]
        }));
    }
}