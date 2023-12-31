mod models;

use std::collections::HashMap;

use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::{get, post};
use rocket::data::Data;
use rocket::serde::json::Json;
use rocket::State;

use sqlx::{PgPool, Postgres, PgConnection, Connection};
use sqlx::postgres::PgPoolOptions;

use serde::Serialize;

struct MyState {
    pool: PgPool
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let state = MyState { pool };
    let rocket = rocket::build()
        .manage(state) 
        .mount("/", rocket::routes![get_events]);

    Ok(rocket.into())
}

use models::Event;
#[get("/events")]
async fn get_events(state: &rocket::State<MyState>) -> Result<Json<Vec<Event>>, Status> {
    let pool = &state.pool;
    match sqlx::query_as::<_, Event>("SELECT * FROM event")
        .fetch_all(pool)
        .await
    {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(Status::InternalServerError),
    }
}

//#[post("/create_event", format="json", data="<data>")]
//fn create_event(pool: &PgPool, data: Json<NewEvent>) -> Status {
//
//    let result = 
//
//    Status::Ok
//}
//

//#[post("/<event_name>/delete")]
//fn delete_event(pool: &State<DatabasePool>, event_name: String) -> Status {
//    let mut conn = pool.get()
//        .expect("ERROR: Failed to get database connection from pool.");
//
//    match diesel::delete(EventTable.filter(schema::event::name.eq(event_name))).execute(&mut conn) {
//        Ok(_) => Status::Ok,
//        Err(_) => Status::BadRequest, // TODO Better Responses 
//    }
//}
//
//#[get("/<event_name>/attendees")]
//fn get_attendees(pool: &State<DatabasePool>, event_name: String) -> Json<Vec<Attendee>> {
//    let mut conn = pool.get()
//        .expect("ERROR: Failed to get database connection from pool");
//    
//    let result =  AttendeeTable
//        .filter(schema::attendee::event_name.eq(event_name))
//        .load::<Attendee>(&mut conn)
//        .expect("ERROR: Failed to load data from attendee table"); // TODO: Error handling
//
//    Json(result)
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
//
//#[post("/<event>/upload_csv", format="text/csv", data="<data>")]
//async fn upload_csv(pool: &State<DatabasePool>, event: &str, data: Data<'_>) -> Result<Status, Status> {
//    
//    #[derive(Serialize, Debug)]
//    enum Values {
//        String(String),
//        Boolean(bool)
//    }
//
//    const MAX_SIZE: i32 = 32 * 1024 * 1024; // TODO: can research and optimize further
//    let string_data = match data.open(rocket::data::ToByteUnit::bytes(MAX_SIZE)).into_string().await {
//        Ok(string) => string,
//        Err(_) => return Err(Status::BadRequest), // ERROR: Could not convert to string
//    };
//
//    let mut rdr = csv::ReaderBuilder::new()
//        .has_headers(true)
//        .from_reader(string_data.as_bytes());
//
//    let mut records = Vec::new();
//    for result in rdr.deserialize() {
//        let record: models::AttendeeCSV = match result {
//            Ok(record) => record,
//            Err(_) => { print!("Yesysatadsadas");
//                return Err(Status::BadRequest); // ERROR: Could not desearlize
//                                            }
//            
//        };
//        records.push(record);
//    };
//
//  let mut connection = pool.get().expect("Connection from pool");
//
//  let Ok((start_date, total_days, total_sessions)) = EventTable
//      .select((schema::event::start_date, schema::event::total_days, schema::event::total_sessions))
//      .first::<(chrono::NaiveDate, i32, i32)>(&mut connection)
//      else {
//          return Err(Status::BadRequest)
//      };
//
//  let mut attendance_log_json = serde_json::json!({
//      "log": []
//  });
//
//  for day in 0..total_days {
//      let date = start_date.checked_add_days(chrono::Days::new(day as u64)).expect("date addtion to work");
//      let final_date = date.format("%d/%m/%Y").to_string();
//      let mut element: HashMap<String, Values> = HashMap::new(); 
//      println!("{:?}", Values::String(String::from("ok")));
//      element.insert(String::from("date"), Values::String(final_date));
//      for i in 0..total_sessions {
//          let session = format!("session{}", i);
//          element.insert(session, Values::Boolean(false));
//      }
//      let log = attendance_log_json["log"].as_array_mut().expect("Yes");
//      let element_obj = serde_json::to_value(element).expect("Hope");
//      log.push(element_obj);
//  }
//    println!("{}", attendance_log_json);
//    Ok(Status::Ok)
//}
