-- This file should undo anything in `up.sql`
ALTER TABLE account
    DROP COLUMN username,
    DROP COLUMN password,
    DROP COLUMN email;

