CREATE TABLE attendee (
    id INT PRIMARY KEY,
    event_name VARCHAR(100) REFERENCES event(event_name),
    name VARCHAR(100),
    email VARCHAR(100),
    roll_number VARCHAR(50),
    attendance_log JSONB,
    misc_log JSONB
);
