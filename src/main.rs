mod models;
mod schema;
mod database;

use schema::event;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::post;

use rocket::data::Data;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![upload_csv])
}

#[post("/<event>/upload_csv", format="text/csv", data="<data>")]
async fn upload_csv(event: String, data: Data<'_>) -> Result<Status, Status> {

    const MAX_SIZE: i32 = 32 * 1024 * 1024;
    let string_data = match data.open(rocket::data::ToByteUnit::bytes(MAX_SIZE)).into_string().await {
        Ok(string) => string,
        Err(_) => return Err(Status::),
    };

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(string_data.as_bytes());

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: models::AttendeeCSV = match result {
            Ok(record) => record,
            Err(_) => {
                return Err(Status::BadRequest)
            }
        };
        records.push(record);
    };

    let mut connection = database::establish_connection();  
    let Ok((start_date, num_of_days, num_of_sessions)) = event::dsl::event.filter(event::event_name.eq(input_event))
                                                   .select((event::starting_date, event::number_of_days, event::number_of_sessions))
                                                   .first::<(chrono::NaiveDate, i32, i32)>(&mut connection)
                                                   else {
                                                       return Err(Status::BadRequest)
                                                   };

    let mut attendance_log_json = json!({
        "log": []
    });
    for day in 0..num_of_days {
        let day: u64 = day as u64;
        let date = start_date.checked_add_days(chrono::Days::new(day));
        let final_date = date.format("%Y-%m-%d").to_string();
        for sessions in 0..num_of_sessions {
            if let Some(log) = attendance_log_json["log"].as_object_mut() {
                log.insert(String::from("date"), serde_json::Value::String(start_date));
            }
        }
    }
    for record in records {
        let attendee_json = json!({
            "id": record.id,
            "name": record.name,
            "email": record.email,
            "roll_number": record.roll_number,
            "misc_log": {
                "log" : []
            }
        });
    };
    Ok(Status::Ok)
}
