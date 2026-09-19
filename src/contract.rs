pub struct Contract {
    pub(crate) id: Uuid,
    pub(crate) drugstore_id: Uuid,
    pub(crate) item_id: Uuid,
    pub(crate) count: i32,
}

impl Contract {
    