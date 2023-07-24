-- Your SQL goes here
-- Query to create an enumeration type Role:
CREATE TYPE Role AS ENUM ('ADMIN', 'DEVELOPER', 'GUEST');

-- Query to modify the users table and add a new column role of type Role:
ALTER TABLE users ADD COLUMN role Role;