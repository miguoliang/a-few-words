CREATE TABLE words (
    word_id SERIAL PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL, -- JWT 'username' field
    word VARCHAR(5000) NOT NULL,
    definition VARCHAR(5000) NOT NULL,
    url VARCHAR(5000) NOT NULL,
    date_added TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    initial_forgetting_rate FLOAT DEFAULT 0.5, -- or another default value
    UNIQUE (user_id, word) -- Ensures each word is unique per user
);

CREATE TABLE review_sessions (
    session_id SERIAL PRIMARY KEY,
    word_id INT REFERENCES words(word_id) ON DELETE CASCADE,
    review_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    recall_score INT CHECK (recall_score BETWEEN 1 AND 5), -- Scale from 1 to 5
    time_to_forget INTERVAL, -- Time before the word is likely forgotten
    next_review_date TIMESTAMP
);

CREATE TABLE forgetting_curve (
    curve_id SERIAL PRIMARY KEY,
    word_id INT REFERENCES words(word_id) ON DELETE CASCADE,
    review_interval INTERVAL, -- Time interval between reviews
    retention_rate FLOAT CHECK (retention_rate BETWEEN 0 AND 1), -- Retention percentage
    review_count INT DEFAULT 0 -- Number of reviews completed
);
