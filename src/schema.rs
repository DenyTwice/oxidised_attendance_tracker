// @generated automatically by Diesel CLI.

diesel::table! {
    attendee (id) {
        id -> Int4,
        #[max_length = 100]
        event_name -> Nullable<Varchar>,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        #[max_length = 50]
        roll_number -> Nullable<Varchar>,
        attendance_log -> Nullable<Jsonb>,
        misc_log -> Nullable<Jsonb>,
    }
}

diesel::table! {
    event (event_name) {
        #[max_length = 100]
        event_name -> Varchar,
        #[max_length = 50]
        starting_date -> Nullable<Varchar>,
        number_of_days -> Nullable<Int4>,
        number_of_sessions -> Nullable<Int4>,
    }
}

diesel::joinable!(attendee -> event (event_name));

diesel::allow_tables_to_appear_in_same_query!(
    attendee,
    event,
);
