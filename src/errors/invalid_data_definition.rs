use std::fmt;

#[derive(Clone)]
pub struct InvalidDataDefinition {
    pub data_key: String,
    pub reason: String,
}

impl fmt::Display for InvalidDataDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid data definition for table {}: {}",
            self.data_key, self.reason
        )
    }
}

impl fmt::Debug for InvalidDataDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, table: {} }}",
            file!(),
            line!(),
            self.data_key
        )
    }
}
