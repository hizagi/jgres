use crate::errors::invalid_data_definition::InvalidDataDefinition;
use crate::errors::invalid_table_definition::InvalidTableDefinition;
use crate::json_loader::entities::JsonStructure;
use serde_json::Value;
use std::collections::BTreeMap;

pub struct DMLProvider {}

impl DMLProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_table_inserts(&self, json_structure: &JsonStructure) -> String {
        let mut query = "".to_owned();
        let dataset = &json_structure.dataset;

        for dataset_item in dataset.iter() {
            let table_query = self
                .data_item_json_to_sql(dataset_item.tablename.to_owned(), &dataset_item.data)
                .unwrap_or_else(|error| panic!("Problem: {:?}", error));
            query.push_str(&table_query);
        }

        let clean_query = str::replace(query.as_str(), "\"", "\'");

        return clean_query;
    }

    fn data_item_json_to_sql(
        &self,
        tablename: String,
        data: &BTreeMap<String, Value>,
    ) -> Result<String, InvalidTableDefinition> {
        if tablename.len() < 1 {
            return Err(InvalidTableDefinition {
                tablename: String::from(tablename.as_str()),
                reason: String::from("No table name"),
            });
        }

        let data_sql: String = match self.data_json_to_sql(data) {
            Ok(data_sql) => data_sql,
            Err(error) => {
                let error = InvalidTableDefinition {
                    tablename: String::from(tablename.as_str()),
                    reason: format!("{} with {}", error.data_key, error.reason),
                };

                print!("Error: {:?}", error);

                return Err(error);
            }
        };

        let sql_str: String = format!("INSERT INTO {} {};", tablename, data_sql);

        return Ok(String::from(sql_str));
    }

    fn data_json_to_sql(
        &self,
        data: &BTreeMap<String, Value>,
    ) -> Result<String, InvalidDataDefinition> {
        let mut attributes_query = "".to_owned();
        let mut values_query = "".to_owned();

        for (key, value) in data.into_iter() {
            if attributes_query.len() < 1 {
                attributes_query = format!("{}", key);
            } else {
                attributes_query = format!("{},{}", attributes_query, key);
            }
            if values_query.len() < 1 {
                values_query = format!("{}", value);
            } else {
                values_query = format!("{},{}", values_query, value);
            }
        }

        let query_string: String = format!("({}) VALUES ({})", attributes_query, values_query);

        Ok(query_string)
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[test]
    fn test_json_structure_to_sql() {
        assert!(true);
    }
}
