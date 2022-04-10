use crate::errors::invalid_attribute_definition::InvalidAttributeDefinition;
use crate::errors::invalid_table_definition::InvalidTableDefinition;
use crate::json_loader::entities::{Attribute, JsonStructure};

pub struct DDLProvider {}

impl DDLProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_create_table(&self, json_structure: JsonStructure) -> String {
        let mut query = "".to_owned();
        let tables = json_structure.tables;

        for table in tables.iter() {
            let table_query = self
                .table_json_to_sql(table.name.to_owned(), &table.attributes)
                .unwrap_or_else(|error| panic!("Problem: {:?}", error));
            query.push_str(&table_query);
        }

        return query;
    }

    fn table_json_to_sql(
        &self,
        tablename: String,
        attributes: &Vec<Attribute>,
    ) -> Result<String, InvalidTableDefinition> {
        if tablename.len() < 1 {
            return Err(InvalidTableDefinition {
                tablename: String::from(tablename.as_str()),
                reason: String::from("No table name"),
            });
        }

        let attributes_sql: String = match self.attributes_json_to_sql(attributes) {
            Ok(attribute_sql) => attribute_sql,
            Err(error) => {
                let error = InvalidTableDefinition {
                    tablename: String::from(tablename.as_str()),
                    reason: format!("{} with {}", error.attribute, error.reason),
                };

                print!("Error: {:?}", error);

                return Err(error);
            }
        };

        let sql_str: String = format!("CREATE TABLE {} ({});", tablename, attributes_sql);

        return Ok(String::from(sql_str));
    }

    fn attributes_json_to_sql(
        &self,
        attributes: &Vec<Attribute>,
    ) -> Result<String, InvalidAttributeDefinition> {
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

    fn attribute_json_to_sql(
        &self,
        attribute: &Attribute,
    ) -> Result<String, InvalidAttributeDefinition> {
        let mut attribute_query = "".to_owned();
        if attribute.name.len() < 1 {
            let error = InvalidAttributeDefinition {
                attribute: String::from(attribute.name.as_str()),
                reason: String::from("No attribute name"),
            };
            print!("Error: {:?}", error);
            return Err(error);
        }

        if attribute.data_type.is_none() {
            let error = InvalidAttributeDefinition {
                attribute: String::from(attribute.name.as_str()),
                reason: String::from("No data type"),
            };

            print!("Error: {:?}", error);
            return Err(error);
        }

        match &attribute.data_type {
            None => {}
            Some(data_type) => {
                attribute_query = format!("{} {}", attribute.name, data_type);
            }
        }

        match &attribute.is_primary {
            None => {}
            Some(is_primary) => {
                if *is_primary {
                    attribute_query = format!("{} PRIMARY KEY", attribute_query);
                }
            }
        }
        match &attribute.not_null {
            None => {}
            Some(not_null) => {
                if *not_null {
                    attribute_query = format!("{} NOT NULL", attribute_query);
                }
            }
        }

        return Ok(attribute_query);
    }
}
