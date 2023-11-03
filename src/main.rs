use std::env;

use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use schema::event;

mod schema;

#[derive(Queryable)]
struct EventEntity {
    event_name: String,
    starting_date: Option<String>, // Consider using chrono::NaiveDate if you want to handle it as a date
    number_of_days: Option<i32>,
    number_of_sessions: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = event)]
struct NewEvent {
    event_name: String,
    starting_date: String,
    number_of_days: i32,
    number_of_sessions: i32,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Database URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let mut connection = establish_connection();

    let new_event = NewEvent {
        event_name: String::from("Test2"),
        starting_date: String::from("10-12-2020"),
        number_of_days: 2,
        number_of_sessions: 2,
    };

    diesel::insert_into(event::table)
        .values(&new_event)
        .execute(&mut connection)
        .expect("Error inserting event");

    let results: Vec<EventEntity> = event::table
        .limit(5)
        .load::<EventEntity>(&mut connection)
        .expect("Error loading results from event");

    for entity in results {
        print!("Found entity {}", entity.event_name);
    }
}
