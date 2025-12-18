-- This file should undo anything in `up.sql`
ALTER TABLE brawlers
DROP CONSTRAINT unique_username UNIQUE (username);