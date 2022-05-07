use std::fmt;

#[derive(Clone)]
pub struct InvalidAttributeDefinition {
    pub attribute: String,
    pub reason: String
}

impl fmt::Display for InvalidAttributeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid attribute definition for attribute {}: {}", self.attribute, self.reason)
    }
}

impl fmt::Debug for InvalidAttributeDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}, table: {} }}", file!(), line!(), self.attribute)
    }
}