CREATE TABLE attendee (
    id INT PRIMARY KEY,
    event_name VARCHAR(100) REFERENCES event(event_name),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    roll_number VARCHAR(50),
    attendance_log JSONB NOT NULL,
    misc_log JSONB NOT NULL
);
