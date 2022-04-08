use crate::json_loader::entities::{JsonStructure, Table, Attribute};
use crate::errors::invalid_table_definition::InvalidTableDefinition;
use crate::errors::invalid_attribute_definition::InvalidAttributeDefinition;

pub struct PostgresAdapter {}

impl PostgresAdapter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn json_structure_to_sql(&self, json_structure: JsonStructure) -> String {
        let mut query = "".to_owned();
        let tables = json_structure.tables;

        for table in tables.iter() {
            let table_query = self.table_json_to_sql(table).unwrap_or_else(|error| {
                panic!("Problem: {:?}", error)
            });
            
            query.push_str(&table_query);
        }

        return query;
    }
    
    pub fn table_json_to_sql(&self, table: &Table) -> Result<String, InvalidTableDefinition> {
        if table.name.len() < 1 {
            return Err(InvalidTableDefinition{
                tablename:  String::from(table.name.as_str()),
                reason: String::from("No table name")
            });
        }

        let sql_str = format!("CREATE TABLE {} (id);", table.name); 

        return Ok(String::from(sql_str));
    }

    pub fn attributes_json_to_sql(&self, attributes: Vec<Attribute>) -> Result<(String, String), InvalidAttributeDefinition> {
        let mut attribute_name_query = "".to_owned();
        let mut attribute_value_query = "".to_owned();
        
        for attribute in attributes.iter() {
            let attribute_id, attribute_type = self.attribute_json_to_sql(attribute).unwrap_or_else(|error| {
                panic!("Problem: {:?}", error)
            });
            
            let query_val = format!() 

            query.push_str(&table_query);
        }
    }
    
    pub fn attribute_json_to_sql(&self, attribute: Attribute) -> Result<(String, String), InvalidAttributeDefinition> {
        if attribute.name.len() < 1 {
            return Err(InvalidAttributeDefinition{
                attribute: attribute.name.as_str(),
                reason: String::from("No attribute name")
            })
        }

        if attribute.data_type.is_some() {
            return Ok(attribute.name, attribute.data_type)
        } else if attribute.native_type.is_some() {
            return Ok(attribute.name, attribute.native_type)
        }

        return Err(InvalidAttributeDefinition{
            attribute: attribute.name.as_str(),
            reason: String::from("No data type and no native type")
        })
    }
}