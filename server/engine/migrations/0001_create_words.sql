CREATE TABLE words (
    id SERIAL PRIMARY KEY,
    word VARCHAR(255) NOT NULL,
    url VARCHAR(255),
    username VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT words_word UNIQUE (word)
);

-- Indexes for quick lookup
CREATE INDEX words_username_idx ON words (username);
CREATE INDEX words_created_at_idx ON words (created_at);
CREATE INDEX words_word_idx ON words (word);

-- Trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_words_updated_at
BEFORE UPDATE ON words
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();