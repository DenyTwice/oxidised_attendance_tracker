mod models;
mod schema;
mod database;

use database::establish_connection;
use models::Event;
use models::{NewEvent, AttendanceInfo};
use schema::event::dsl::*;
use schema::event::table as EventTable;

use diesel::RunQueryDsl;
use rocket::http::Status;
use rocket::{get, post};
use rocket::data::Data;

use rocket::serde::json::Json;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[rocket::launch]
fn rocket() -> _ {

    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("Database URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    rocket::build()
        .manage(pool)
        .mount("/", rocket::routes![create_event, get_events])
}

#[post("/create_event", format="json", data="<data>")]
fn create_event(pool: &rocket::State<DbPool>, data: Json<NewEvent>) -> Status {
    
    let new_event: NewEvent = data.into_inner();
    let other_event: models::Event = models::Event::from(new_event);

    let mut connection = pool.get().expect("TODO");

    diesel::insert_into(EventTable)
        .values(&other_event)
        .execute(&mut connection)
        .expect("TODO");

    Status::Ok
}

#[get("/events")]
fn get_events() -> Json<Vec<Event>> {
    let mut conn = establish_connection();

    let result = EventTable
        .load::<Event>(&mut conn)
        .expect("Load data from eventtable in get_events");

    Json(result)
}
//
//#[post("/<event>/delete")]
//fn delete_event(event: String) -> String {
//    format!("deleted {} ", event)
//}
//
//#[get("/<event>/attendees")]
//fn get_attendees(event: String) -> String {
//    format!("get from {}", event)
//}
//
//#[post("/<event>/mark_attendance", format="json", data="<data>")]
//fn mark_attendance(event: String, data: Json<AttendanceInfo>) -> String {
//    format!("attendee of event {}", event)
//}
//
//#[get("/<event>/present_attendees")] 
//fn get_present_attendees(event: String) -> String {
//    format!("yes of this {}", event)
//}

//#[post("/<event>/upload_csv", format="text/csv", data="<data>")]
//async fn upload_csv(event: String, data: Data<'_>) -> Result<Status, Status> {
//
//    const MAX_SIZE: i32 = 32 * 1024 * 1024; // TODO: can research and optimize further
//    let string_data = match data.open(rocket::data::ToByteUnit::bytes(MAX_SIZE)).into_string().await {
//        Ok(string) => string,
//        Err(_) => return Err(Status::BadRequest), // ERROR: Could not convert to string
//    };
//
//    let mut rdr = csv::ReaderBuilder::new()
//        .has_headers(false)
//        .from_reader(string_data.as_bytes());
//
//    let mut records = Vec::new();
//    for result in rdr.deserialize() {
//        let record: models::AttendeeCSV = match result {
//            Ok(record) => record,
//            Err(_) => return Err(Status::BadRequest) // ERROR: Could not desearlize
//            
//        };
//        records.push(record);
//    };
//
    //let mut connection = database::establish_connection();  

    //let Ok((start_date, num_of_days, num_of_sessions)) = event::dsl::event.filter(event::event_name.eq(event))
    //                                               .select((event::starting_date, event::number_of_days, event::number_of_sessions))
    //                                               .first::<(chrono::NaiveDate, i32, i32)>(&mut connection)
    //                                               else {
    //                                                   return Err(Status::BadRequest)
    //                                               };

    //let mut attendance_log_json = json!({
    //    "log": []
    //});
    //for day in 0..num_of_days {
    //    let day: u64 = day as u64;
    //    let date = start_date.checked_add_days(chrono::Days::new(day));
    //    let final_date = date.format("%Y-%m-%d").to_string();
    //    for sessions in 0..num_of_sessions {
    //        if let Some(log) = attendance_log_json["log"].as_object_mut() {
    //            log.insert(String::from("date"), serde_json::Value::String(start_date));
    //        }
    //    }
    //}
    //for record in records {
    //    let attendee_json = json!({
    //        "id": record.id,
    //        "name": record.name,
    //        "email": record.email,
    //        "roll_number": record.roll_number,
    //        "misc_log": {
    //            "log" : []
    //        }
    //    });
    //};
    
//    Ok(Status::Ok)
//}
