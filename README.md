# Oxidised Attendance Tracker (Backend)

Backend for [Aaryanajith/AttendenceTracker_Frontend](https://github.com/Aaryanajith/AttendenceTracker_Frontend), 
a Flutter app for keeping track of attendees at events.

Made with Rocket.rs and Diesel. 

## Installing

Refer to [The Rust Book](https://doc.rust-lang.org/cargo/getting-started/installation.html) for how to install Rust and Cargo using Rustup.
Additional Rust crates and dependencies will be automatically downloaded and compiled on initial run.

Install Diesel CLI to run and manage database migrations:
`cargo install diesel_cli --no-default-features --features postgres`

## Building and Running

### Step 1: Setup database
Create a Postgres database on your machine and use an environment file to specify the database URL like so:
` DATABASE_URL=postgresql://<user_name>:<password>@localhost:5432/<database_name>`

### Step 2: Run Diesel Migrations
Run migrations to create required tables in your database:
`diesel migrations run`

Or revert both migrations using `diesel migration revert`
and then run them as stated above.

### Step 3: Run/Build
Use `cargo run` to start the server and `cargo build` to get the compiled binary.

## API Documentation

### Create an event
- **Endpoint**: `POST /create_event`
- **Description**: Creates a new event.
- **Response**:
    - `201 OK`: Returns the created event object.
    - `400 Bad Request`: If the request JSON was malformed.
    - `422 Unprocessable Entity`: Server failed to process the JSON.
    - `500 Internal Server Error`: Server failed to insert the entity.
- **Request Body**: 
```json
{
	"name": "String",
	"start_date": "String in %d/%m/%Y format."
	"total_days": "integer",
	"total_sessions": "integer" 
}
```
- **Example**:
```shell
curl --request POST \
  --url http://127.0.0.1:8000/create_event \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Test Request",
	"start_date": "01/01/2000",
	"total_days": 2,
	"total_sessions": 2
}'```

## Delete an Event

- **Endpoint**: `POST /<event_name>/delete`
- **Description**: Deletes an event specified by its name.
- **Response**:
  - `204 No Content`: Event successfully deleted.
  - `400 Bad Request`: Failed to delete due to incorrect event name.
  - `500 Internal Server Error`: Server error during deletion.
- **URL Parameter**: `event_name` (string) - The name of the event to be deleted.
- **Example**:
```shell
curl --request POST \
  --url http://127.0.0.1:8000/Test%20Request/delete
```

## Get Attendees of an Event

- **Endpoint**: `GET /<event_name>/attendees`
- **Description**: Retrieves the list of attendees for a specified event.
- **Response**:
  - `200 OK`: Successfully returns a list of attendees.
  - `400 Bad Request`: Failed to delete due to incorrect event name.
  - `500 Internal Server Error`: Server error during data retrieval.
- **URL Parameter**: `event_name` (string) - The name of the event.
- **Example**:
```shell
curl --request GET \
  --url http://127.0.0.1:8000/Test%20Request/attendees \
```

## Upload CSV data of attendees

- **Endpoint**: `POST /<event>/upload_csv`
- **Description**: Uploads a CSV file for inserting attendees for the specified event.
- **Response**:
  - `200 OK`: CSV file successfully processed.
  - `400 Bad Request`: Failed to process the CSV file.
  - `500 Internal Server Error`: Server error during processing.
- **URL Parameter**: `event` (string) - The name of the event for which the CSV is uploaded.
- **Request Body**: `text/csv` formatted data.
```csv
id,name,email,roll_number
1,Paul Allen,paullallen@hotmail.com,AM.EN.U4NYC000
2,Luigi,luigi@gmailcom,AM.EN.MARIO123
```
Or a .csv file.
- **Example**:
```shell
curl --request POST \
  --url http://127.0.0.1:8000/Test%20Request/upload_csv \
  --header 'Content-Type: text/csv' \
  --data 'id,name,email,roll_number
1,Paul Allen,paullallen@hotmail.com,AM.EN.U4NYC000
2,Luigi,luigi@gmailcom,AM.EN.MARIO123'
```
