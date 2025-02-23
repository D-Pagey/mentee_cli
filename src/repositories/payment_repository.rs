use rusqlite::{params, Connection};

use crate::constants;

pub struct PaymentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn delete_payment(&self, payment_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::PAYMENTS_TABLE);

        self.conn.execute(&sql, params![payment_id])
    }
}
