-- Your SQL goes here
CREATE TABLE users (
    id SERIAL primary key not null,
    email varchar not null unique,
    password varchar not null 
)