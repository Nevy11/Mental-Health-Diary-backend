-- Your SQL goes here
CREATE TABLE goals_done(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    goal_name VARCHAR NOT NULL
);