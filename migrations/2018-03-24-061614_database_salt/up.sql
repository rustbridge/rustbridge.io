-- Your SQL goes here
CREATE TABLE IF NOT EXISTS salts (
  id SERIAL primary key not null,
  salt varchar not null unique
)
