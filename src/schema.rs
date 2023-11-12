// @generated automatically by Diesel CLI.

diesel::table! {
    attendee (id) {
        #[max_length = 100]
        event_name -> Varchar,
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 30]
        roll_number -> Nullable<Varchar>,
        attendance_log -> Jsonb,
        misc_log -> Jsonb,
    }
}

diesel::table! {
    event (name) {
        #[max_length = 100]
        name -> Varchar,
        start_date -> Date,
        total_days -> Int4,
        total_sessions -> Int4,
    }
}

diesel::joinable!(attendee -> event (event_name));

diesel::allow_tables_to_appear_in_same_query!(
    attendee,
    event,
);
