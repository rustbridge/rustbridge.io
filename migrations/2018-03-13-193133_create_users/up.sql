-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL primary key not null,
    email varchar not null unique,
    password varchar not null 
)
