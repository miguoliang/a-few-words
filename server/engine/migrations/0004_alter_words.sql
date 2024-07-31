ALTER TABLE words ADD COLUMN username varchar(255);
ALTER TABLE words ADD COLUMN created_at timestamp NOT NULL DEFAULT now();
CREATE INDEX words_username ON words (username);
CREATE INDEX words_created_at ON words (created_at);
CREATE INDEX words_word ON words (word);