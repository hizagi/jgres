use pgx::*;

pub mod json_loader;
pub mod errors;

use crate::json_loader::provider::JsonProvider;

pg_module_magic!();

#[pg_extern]
fn run_json(path: &str) {
    let json_provider = JsonProvider::new(path);

    let sql: String = json_provider.load_json();

    Spi::run(sql.as_str())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_run_json() {
        crate::run_json("/home/hizagi/projects/jgres/test.json")
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
