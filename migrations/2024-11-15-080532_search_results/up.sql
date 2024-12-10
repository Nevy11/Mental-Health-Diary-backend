-- Your SQL goes here
CREATE TABLE search_results(
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    link TEXT NOT NULL,
    snippet TEXT
);