use rusqlite::Connection;

use crate::{error::MenteeError, repositories::PaymentRepository};

pub struct PaymentService<'a> {
    payment_repo: PaymentRepository<'a>,
}

impl<'a> PaymentService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            payment_repo: PaymentRepository::new(conn),
        }
    }

    pub fn delete_payment(&self, payment_id: u32) -> Result<String, MenteeError> {
        match self.payment_repo.delete_payment(payment_id) {
            Ok(0) => Err(MenteeError::NotFound(format!(
                "Payment with id {}",
                payment_id
            ))),
            Ok(_) => Ok(format!("Payment with id {} deleted", payment_id)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }
}
