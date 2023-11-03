// @generated automatically by Diesel CLI.

diesel::table! {
    attendee (id) {
        id -> Int4,
        #[max_length = 100]
        event_name -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 50]
        roll_number -> Nullable<Varchar>,
        attendance_log -> Jsonb,
        misc_log -> Jsonb,
    }
}

diesel::table! {
    event (event_name) {
        #[max_length = 100]
        event_name -> Varchar,
        starting_date -> Date,
        number_of_days -> Int4,
        number_of_sessions -> Int4,
    }
}

diesel::joinable!(attendee -> event (event_name));

diesel::allow_tables_to_appear_in_same_query!(
    attendee,
    event,
);
