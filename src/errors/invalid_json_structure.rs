use std::fmt;

#[derive(Clone)]
pub struct InvalidJsonStructure {
    error_msg: String
}

impl fmt::Display for InvalidJsonStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid table definition for table: {}", self.error_msg)
    }
}

impl fmt::Debug for InvalidJsonStructure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {}, table: {} }}", file!(), line!(), self.error_msg)
    }
}