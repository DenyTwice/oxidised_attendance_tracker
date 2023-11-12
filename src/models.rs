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

impl From<NewEvent> for Event {
    fn from(value: NewEvent) -> Self {
        const DATE_FORMAT: &str = "%d/%m/%Y";
        let date = NaiveDate::parse_from_str(&value.start_date, DATE_FORMAT).expect("Date conversion from string to Naivedate");
        Event {
            name: value.name,
            start_date: date,
            total_days: value.total_days,
            total_sessions: value.total_sessions
        }
    }
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
