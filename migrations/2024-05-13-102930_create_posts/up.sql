-- Your SQL goes here
CREATE TABLE conversations (
    id TEXT NOT NULL PRIMARY KEY,
    user1_id TEXT NOT NULL,
    user2_id TEXT NOT NULL
);


CREATE TABLE messages (
id          SERIAL PRIMARY KEY,
user_id    TEXT NOT NULL,
message     TEXT NOT NULL,
conversation_id TEXT NOT NULL REFERENCES conversations(id)
);

CREATE TABLE users (
id          SERIAL PRIMARY KEY,
username    TEXT NOT NULL,
first_name  TEXT NOT NULL,
last_name   TEXT NOT NULL,
email       TEXT NOT NULL UNIQUE CHECK (POSITION('@' IN EMAIL) > 1),
password    TEXT NOT NULL,
img_url     TEXT,
is_admin    BOOLEAN DEFAULT FALSE
);


CREATE TABLE flagged(
id         SERIAL PRIMARY KEY, 
email      TEXT NOT NULL UNIQUE CHECK (POSITION('@' IN EMAIL) > 1), 
reason     TEXT NOT NULL
); 