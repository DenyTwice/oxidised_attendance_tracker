CREATE TABLE attendee (
    event_name VARCHAR(100) REFERENCES event(name),
    id INT PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    email VARCHAR(150) NOT NULL,
    roll_number VARCHAR(30),
    attendance_log JSONB NOT NULL,
    misc_log JSONB NOT NULL
);
