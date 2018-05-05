-- Your SQL goes here
CREATE TABLE salts (
  id SERIAL primary key not null,
  salt varchar not null unique,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('salts');
