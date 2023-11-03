#![feature(decl_macro)]
extern crate rocket;
use rocket::{get, routes, launch};
extern crate chrono;
use std::env;

use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use schema::event;

mod schema;

#[derive(Queryable)]
struct EventEntity {
    event_name: String,
    starting_date: NaiveDate,
    number_of_days: i32,
    number_of_sessions: i32,
}

#[derive(Insertable)]
#[diesel(table_name = event)]
struct NewEvent {
    event_name: String,
    starting_date: NaiveDate,
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

fn test() {
    let mut connection = establish_connection();
    let date = NaiveDate::parse_from_str("2023-10-12", "%Y-%m-%d").expect("Hardcode date must be valid");
    let new_event = NewEvent {
        event_name: String::from("Test4"),
        starting_date: date,
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


#[get("/")]
fn index() {

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
