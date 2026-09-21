-- -- Add migration script here
CREATE TABLE medicine IF NOT EXISTS (
    id              BLOB PRIMARY KEY,
    name            TEXT NOT NULL,
    maker           TEXT NOT NULL,
    PRICE           REAL NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT
);