-- Your SQL goes here
CREATE TABLE questions(
    id SERIAL PRIMARY KEY,
    question VARCHAR NOT NULL,
    context VARCHAR NOT NULL
);