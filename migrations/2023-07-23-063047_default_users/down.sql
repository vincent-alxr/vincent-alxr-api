-- This file should undo anything in `up.sql`
-- Step 1: Delete the user with the email 'aurelien@alixir.fr' and role 'ADMIN'
DELETE FROM users WHERE email = 'aurelien@alixir.fr' AND role = 'ADMIN';