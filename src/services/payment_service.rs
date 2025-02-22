use rusqlite::Connection;

use crate::repositories::PaymentRepository;

pub struct PaymentService<'a> {
    payment_repo: PaymentRepository<'a>,
}

impl<'a> PaymentService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            payment_repo: PaymentRepository::new(conn),
        }
    }
}
