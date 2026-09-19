pub struct Contract {
    pub(crate) id: Uuid,
    pub(crate) drugstore_id: Uuid,
    pub(crate) item_id: Uuid,
    pub(crate) count: i32,
}

impl Contract {
    pub fn create_new(drugstore_id: &Drugstore, item_id: &Item, count: i32) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            drugstore_id,
            item_id,
        }
    }

    pub fn create(id: Uuid, drugstore_id: &Drugstore, item_id: &Item, count: i32) -> Self {
        let result: Contract = Self {
            id,
            drugstore_id,
            item_id,
        };
        result
    }
}
