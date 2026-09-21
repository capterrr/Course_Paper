-- Add migration script here
-- pub struct Pharmacy {
--     pub(crate) id: Uuid,
--     pub(crate) name: String,
--     pub(crate) address: String,
--     pub(crate) phone: String,
--     pub(crate) created_at: UtcDateTime,
--     pub(crate) updated_at: Option<UtcDateTime>,
--     pub(crate) deleted_at: Option<UtcDateTime>,
-- }


CREATE TABLE IF NOT EXISTS pharmacy (
    id              BLOB NOT NULL PRIMARY KEY,
    name            TEXT NOT NULL,
    address         TEXT NOT NULL,
    phone           TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT,
    deleted_at      TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_phone_number ON pharmacy (phone);

