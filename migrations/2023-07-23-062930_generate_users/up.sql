-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    bio TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    deleted_at timestamp
)