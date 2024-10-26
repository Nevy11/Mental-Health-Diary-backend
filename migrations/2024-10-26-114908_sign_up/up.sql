-- Your SQL goes here
DROP TABLE IF EXISTS chat_users_table;
DROP TABLE IF EXISTS mental_users_init;
CREATE TABLE chat_users(
    username VARCHAR PRIMARY KEY,
    userpassword VARCHAR NOT NULL, 
    email VARCHAR NOT NULL
)