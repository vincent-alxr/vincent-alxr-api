-- This file should undo anything in `up.sql`

-- Step 1: Remove the role column from the users table
ALTER TABLE users DROP COLUMN role;

-- Step 2: Drop the Role enum type
DROP TYPE Role;