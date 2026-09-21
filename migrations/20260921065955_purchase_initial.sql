-- Add migration script here
-- pub struct Purchase {
--     pub(crate) order_id: Uuid,
--     pub(crate) medicine_id: Uuid,
--     pub(crate) quantity: i32,
--     pub(crate) created_at: UtcDateTime,
-- }



CREATE TABLE IF NOT EXISTS purchase (
    id                  BLOB NOT NULL PRIMARY KEY,
    medicine_id         BLOB NOT NULL,
    order_id            BLOB NOT NULL,
    quantity            INTEGER NOT NULL,
    created_at          TEXT NOT NULL
);