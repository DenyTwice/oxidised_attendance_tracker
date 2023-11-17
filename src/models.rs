use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Event {
    pub name: String,
    pub start_date: String,
    pub total_days: i32,
    pub total_sessions: i32,
}
