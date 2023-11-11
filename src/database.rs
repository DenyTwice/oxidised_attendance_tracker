pub fn establish_connection() -> diesel::PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL must be set");

    diesel::Connection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
