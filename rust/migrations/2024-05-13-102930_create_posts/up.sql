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
is_admin    BOOLEAN DEFAULT FALSE,
img_url     TEXT DEFAULT 'https://fastly.picsum.photos/id/0/5000/3333.jpg?hmac=_j6ghY5fCfSD6tvtcV74zXivkJSPIfR9B8w34XeQmvU' NOT NULL
);


CREATE TABLE flagged(
id         SERIAL PRIMARY KEY, 
email      TEXT NOT NULL UNIQUE CHECK (POSITION('@' IN EMAIL) > 1), 
reason     TEXT NOT NULL
); 