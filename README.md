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
- Endpoint: `POST /create_event`
- Description: Creates a new event.
- Response:
    - `201 OK`: Returns the created event object.
    - `400 Bad Request`: If the request JSON was malformed.
    - `422 Unprocessable Entity`: Server failed to process the JSON.
    - `500 Internal Server Error`: Server failed to insert the entity.
- Request Body: 
```json
{
	"name": "String",
	"start_date": "String in %d/%m/%Y format."
	"total_days": "integer",
	"total_sessions": "integer" 
}
```
- Example:
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

