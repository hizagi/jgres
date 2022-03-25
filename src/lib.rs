use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_jgres() -> &'static str {
    "Hello, jgres"
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
