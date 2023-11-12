CREATE TABLE attendee (
    event_name VARCHAR(100) REFERENCES event(name) ON DELETE CASCADE NOT NULL,
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(150) NOT NULL,
    email VARCHAR(150) NOT NULL,
    roll_number VARCHAR(30),
    attendance_log JSONB NOT NULL,
    misc_log JSONB NOT NULL
);
