use uuid::Uuid;
use time::UtcDateTime;
use crate::order::Order;
use crate::medicine::Medicine;

#[doc = "Сущность Закупка лекарственных препаратов"]
pub struct Purchase {
    pub(crate) order_id: Uuid,
    pub(crate) medicine_id: Uuid,
    pub(crate) quantity: i32,
    pub(crate) created_at: UtcDateTime,
}

impl Purchase {
    
    pub fn create_new(order: &Order, medicine: &Medicine, quantity: i32) -> Self {
        Self {
            order_id: order.id,
            medicine_id: medicine.id,
            quantity,
            created_at: UtcDateTime::now_utc(),
        }
    }

    pub fn create(order: &Order, medicine: &Medicine, quantity: i32, created_at: UtcDateTime) -> Self {
        Self {
            order_id: order.id,
            medicine_id: medicine.id,
            quantity,
            created_at,
        }
    }
}
