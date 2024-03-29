-- Your SQL goes here
CREATE TABLE messages (
id          SERIAL PRIMARY KEY,
user_id     TEXT NOT NULL,
message     TEXT NOT NULL
);

CREATE TABLE users (
id          SERIAL PRIMARY KEY,
is_admin    BOOLEAN DEFAULT FALSE,
username    TEXT NOT NULL,
first_name  TEXT NOT NULL,
last_name   TEXT NOT NULL,
email       TEXT NOT NULL UNIQUE CHECK (POSITION('@' IN EMAIL) > 1),
password    TEXT NOT NULL,
created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);