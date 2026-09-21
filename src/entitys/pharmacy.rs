use uuid::Uuid;
use time::UtcDateTime;

#[doc = "Сущность Аптека"]
pub struct Pharmacy {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Pharmacy {
    pub fn create_new(name: String, address: String, phone: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            address,
            phone,
            created_at: UtcDateTime::now_utc(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        name: String,
        address: String,
        phone: String,
        created_at: UtcDateTime,
    ) -> Self {
        Self {
            id,
            name,
            address,
            phone,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }
}
