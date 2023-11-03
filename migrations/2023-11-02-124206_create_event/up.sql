CREATE TABLE event (
    event_name VARCHAR(100) PRIMARY KEY,
    starting_date DATE NOT NULL,
    number_of_days INT NOT NULL,
    number_of_sessions INT NOT NULL
);
