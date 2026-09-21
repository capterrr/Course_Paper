-- -- Add migration script here
CREATE TABLE  IF NOT EXISTS medicine (
    id              BLOB PRIMARY KEY NOT NULL,
    name            TEXT NOT NULL,
    maker           TEXT NOT NULL,
    PRICE           REAL NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT
);