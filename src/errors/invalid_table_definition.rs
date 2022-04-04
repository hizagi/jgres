use std::fmt;

#[derive(Clone)]
pub struct InvalidTableDefinition {
    tablename: String
    reason: String
}

impl fmt::Display for InvalidTableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid table definition for table {}: {}", self.tablename, self.reason)
    }
}

impl fmt::Debug for InvalidTableDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}, table: {} }}", file!(), line!(), self.tablename)
    }
}