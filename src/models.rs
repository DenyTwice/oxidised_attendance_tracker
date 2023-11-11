use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use super::schema::{event, attendee};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = event)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub name: String,
    pub start_date: NaiveDate,
    pub total_days: i32,
    pub total_sessions: i32,
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewEvent {
    pub name: String,
    pub start_date: String,
    pub total_days: i32,
    pub total_sessions: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AttendanceInfo {
    pub date: String,
    pub time: String,
    pub session: String,
    pub id: String
}
