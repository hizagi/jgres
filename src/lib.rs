use pgx::*;
use serde::{Deserialize, Serialize};

pg_module_magic!();

#[derive(Serialize, Deserialize)]
struct JsonStructure {
    tables: Vec[Table],
}

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    attributes: Vec[Attribute],
}

#[derive(Serialize, Deserialize)]
struct Attribute {
    name: String,
    data_type: String,
    native_type: String,
}

#[pg_extern]
fn runJson(path: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let parsed: JsonStructure = serde_json::from_str(&contents).unwrap();
    return parsed.success
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_jgres() {
        assert_eq!("Hello, jgres", crate::hello_jgres());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
