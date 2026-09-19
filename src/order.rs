use uuid::Uuid;
use time::UtcDateTime;
use crate::pharmacy::Pharmacy;

#[doc = "Сущность Заявка"]
pub struct Order {
    pub(crate) id: Uuid,
    pub(crate) pharmacy_id: Uuid,
    pub(crate) created_at: UtcDateTime,           // Дата составления заявки
    pub(crate) finished_at: Option<UtcDateTime>,  // Дата выполнения заявки
}

impl Order {
    // Каждая заявка обязательно имеет отношение к некоторой аптеке
    pub fn create_new(pharmacy: &Pharmacy) -> Self {
        Self {
            id: Uuid::new_v4(),
            pharmacy_id: pharmacy.id,
            created_at: UtcDateTime::now_utc(),
            finished_at: None,
        }
    }

    pub fn create(
        id: Uuid,
        pharmacy: &Pharmacy,
        created_at: UtcDateTime,
        finished_at: Option<UtcDateTime>,
    ) -> Self {
        Self {
            id,
            pharmacy_id: pharmacy.id,
            created_at,
            finished_at,
        }
    }
}

