-- Your SQL goes here
ALTER TABLE brawlers
ADD CONSTRAINT unique_username UNIQUE (username);