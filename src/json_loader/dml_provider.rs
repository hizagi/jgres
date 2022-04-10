use crate::errors::invalid_attribute_definition::InvalidAttributeDefinition;
use crate::errors::invalid_table_definition::InvalidTableDefinition;
use crate::json_loader::entities::{Attribute, Data, JsonStructure, TableContent, Tables};

pub struct DMLProvider {}

impl DMLProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_table_inserts(&self, json_structure: JsonStructure) -> String {
        let mut query = "".to_owned();
        let dataset: Vec<Data> = json_structure.dataset;
        let tables: Tables = json_structure.dataset;

        for (idx, data) in dataset.iter() {
            let tablename: String = idx.to_owned();
            let table_content: TableContent;
            match tables.get(&data.tablename) {
                None => {}
                Some(value) => table_content = value,
            }

            print!("{:?} ,\n {:?}", table_content, data)
            // let dataset_query = self.data_json_to_sql(&table_content, data.data).unwrap_or_else(|error| {
            //     panic!("Problem: {:?}", error)
            // });
            // query.push_str(&dataset_query);
        }

        return query;
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_json_structure_to_sql() {
        assert!(false);
    }
}
