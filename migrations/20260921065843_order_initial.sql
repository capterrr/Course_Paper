-- Add migration script here
-- pub struct Order {
--     pub(crate) id: Uuid,
--     pub(crate) pharmacy_id: Uuid,
--     pub(crate) created_at: UtcDateTime,           // Дата составления заявки
--     pub(crate) finished_at: Option<UtcDateTime>,
-- }

CREATE TABLE IF NOT EXISTS orders (
    
    id              BLOB PRIMARY KEY,
    pharmacy_id     BLOB NOT NULL,
    created_at      TEXT NOT NULL,
    finished_at     TEXT,
    deleted_at      TEXT
);