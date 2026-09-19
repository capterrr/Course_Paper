pub struct drugstore {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

impl Drugstore {
    pub fn create_new(id: Uuid) {
        let id = Uuid::new_v4();
        Self {
            id: Uuid,
            name: String,
            address: String,
            phone: String,
        }
    }
    pub fn create(id: Uuid, name: String, address: String, phone: String) -> Self {
        let result: Drugstore = Self {
            id,
            name,
            address,
            phone,
        };
        result
    }
}
