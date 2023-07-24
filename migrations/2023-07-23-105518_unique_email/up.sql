-- Your SQL goes here
CREATE UNIQUE INDEX user_unique_email ON users(email) WHERE deleted_at IS NULL;