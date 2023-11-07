use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use super::schema::{event, attendee};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = event)]
pub struct Event {
    pub event_name: String,
    pub starting_date: NaiveDate,
    pub number_of_days: i32,
    pub number_of_sessions: i32,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = attendee)]
pub struct Attendee {
    pub id: i32,
    pub event_name: String,
    pub name: String,
    pub email: String,
    pub roll_number: String,
    pub attendance_log: JsonValue,
    pub misc_log: JsonValue,
}

#[derive(Deserialize)]
pub struct AttendeeCSV {
    pub id: i32,
    pub event_name: String,
    pub name: String,
    pub email: String,
    pub roll_number: String,
}
