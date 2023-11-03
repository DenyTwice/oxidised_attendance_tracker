extern crate chrono;
use chrono::Naviedate;
mod schema;
use crate::schema::attendee;
use crate::schema::event;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = event)]
pub struct Event {
    pub event_name: String,
    pub starting_date: Naviedate, // Consider using chrono::NaiveDate if you want to handle it as a date
    pub number_of_days: i32,
    pub number_of_sessions: i32,
}


#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = attendee)]
pub struct Participant {
    pub id: i32,
    pub event_name: String,
    pub name: String,
    pub email: String,
    pub roll_number: String,
    pub attendance_log: JsonValue,
    pub misc_log: JsonValue,
}
