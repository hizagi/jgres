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
    
    fn table_json_to_sql(&self, table: &Table) -> Result<String, InvalidTableDefinition> {
        if table.name.len() < 1 {
            return Err(InvalidTableDefinition{
                tablename:  String::from(table.name.as_str()),
                reason: String::from("No table name")
            });
        }

        let attributes_sql: String = match self.attributes_json_to_sql(&table.attributes) {
            Ok(attribute_sql) => attribute_sql, 
            Err(error) => return Err(
                InvalidTableDefinition{
                    tablename: String::from(table.name.as_str()),
                    reason: format!("{} with {}", error.attribute, error.reason)
                }
            ),
        };

        let sql_str: String = format!("CREATE TABLE {} ({});", table.name, attributes_sql); 

        return Ok(String::from(sql_str));
    }

    fn attributes_json_to_sql(&self, attributes: &Vec<Attribute>) -> Result<String, InvalidAttributeDefinition> {
        let mut attributes_query = "".to_owned();
        
        for attribute in attributes.iter() {
            let attribute_query_result: String = match self.attribute_json_to_sql(attribute) {
                Ok(attribute_sql) => attribute_sql, 
                Err(error) => return Err(error),
            };
            
            if attributes_query.len() < 1 {
                attributes_query = format!("{}", attribute_query_result);
            } else {
                attributes_query = format!("{},{}", attributes_query, attribute_query_result);
            }
        }

        return Ok(attributes_query);
    }
    
    fn attribute_json_to_sql(&self, attribute: &Attribute) -> Result<String, InvalidAttributeDefinition> {
        let mut attribute_query = "".to_owned();
        
        if attribute.name.len() < 1 {
            return Err(InvalidAttributeDefinition{
                attribute: String::from(attribute.name.as_str()),
                reason: String::from("No attribute name")
            })
        }

        if attribute.data_type.is_none() {
            return Err(InvalidAttributeDefinition{
                attribute: String::from(attribute.name.as_str()),
                reason: String::from("No data type")
            })
        }

        match &attribute.data_type {
            None => {},
            Some(data_type) => {
                attribute_query = format!("{} {}", attribute.name, data_type);
            },
        }

        match &attribute.is_primary {
            None => {},
            Some(is_primary) => {
                if *is_primary {
                    attribute_query = format!("{} PRIMARY KEY", attribute_query);
                }
            }, 
        }
        
        match &attribute.not_null {
            None => {},
            Some(not_null) => {
                if *not_null {
                    attribute_query = format!("{} NOT NULL", attribute_query);
                }
            }, 
        }

        return Ok(attribute_query)
    }

}