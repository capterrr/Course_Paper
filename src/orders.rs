pub struct Order {
    pub(crate) id: Uuid,
    pub(crate) date: UtcDateTime,
    pub(crate) drugstore_id: Uuid,
    pub(crate) finihed_at: Option<UtcDateTime>,
}
