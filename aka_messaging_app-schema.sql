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

CREATE TABLE group (
id           SERIAL PRIMARY KEY,
name 	     TEXT NOT NULL,
participants TEXT NOT NULL,
created_at   TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE messages (
id          SERIAL PRIMARY KEY,
room_id     INTEGER NOT  NULL,
user_id     INTEGER NOT  NULL,
message     TEXT NOT NULL,
created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

