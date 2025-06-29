-- Add up migration script here
CREATE TABLE commands (
    id SERIAL PRIMARY KEY,
    command TEXT NOT NULL,
    output TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);