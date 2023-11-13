# Oxidised Attendance Tracker (Backend)

Backend for [Aaryanajith/AttendenceTracker_Frontend](https://github.com/Aaryanajith/AttendenceTracker_Frontend), 
a Flutter app for keeping track of attendees at events.

Made Rocket.rs and Diesel. 

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

### Step 3: Run
Use `cargo run` to start the server and `cargo build` to get the compiled binary.

## API Documentation
