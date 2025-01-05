#[cfg(test)]
pub fn establish_test_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
