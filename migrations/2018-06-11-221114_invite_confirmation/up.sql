-- Your SQL goes here
CREATE TABLE invite_confirmations (
  id SERIAL PRIMARY KEY NOT NULL,
  code VARCHAR NOT NULL UNIQUE,
  invite_id INTEGER NOT NULL REFERENCES invites(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('invite_confirmations');
