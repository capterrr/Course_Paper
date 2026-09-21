use uuid::Uuid;
use time::UtcDateTime;

#[doc = "Сущность Лекарство"]
pub struct Medicine {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) maker: String,
    pub(crate) price: f64,
    pub(crate) created_at: UtcDateTime,
    pub(crate) updated_at: Option<UtcDateTime>,
    pub(crate) deleted_at: Option<UtcDateTime>,
}

impl Medicine {
    pub fn create_new(name: String, maker: String, price: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            maker,
            price,
            created_at: UtcDateTime::now_utc(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        name: String,
        maker: String,
        price: f64,
        created_at: UtcDateTime,
    ) -> Self {
        Self {
            id,
            name,
            maker,
            price,
            created_at,
            updated_at: None,
            deleted_at: None,
        }
    }
}
