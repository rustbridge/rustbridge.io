-- Your SQL goes here
CREATE TABLE invites (
  id SERIAL PRIMARY KEY NOT NULL,
  workshop_id INTEGER NOT NULL REFERENCES workshops(id),
  email VARCHAR NOT NULL UNIQUE,
  attending BOOLEAN NOT NULL,
  pending BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('invites');
