-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL primary key not null,
    session_key varchar not null unique,
    user_id int references users(id)
);
