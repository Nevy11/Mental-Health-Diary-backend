-- Your SQL goes here
CREATE TABLE ai(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    question VARCHAR NOT NULL,
    answer VARCHAR NOT NULL
);