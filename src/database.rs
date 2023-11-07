pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL must be set");

    diesel::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

