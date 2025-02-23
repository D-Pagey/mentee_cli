use rusqlite::{params, Connection};

use crate::{constants, models::payment::Payment};

pub struct PaymentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_payment_by_id(&self, id: u32) -> Result<Payment, rusqlite::Error> {
        let sql = format!("SELECT * FROM {} WHERE id = ?1", constants::PAYMENTS_TABLE);

        self.conn.query_row(&sql, params![id], |row| {
            Ok(Payment {
                id: row.get(0)?,
                mentee_id: row.get(1)?,
                date: row.get(2)?,
                amount: row.get(3)?,
            })
        })
    }

    pub fn update_payment(
        &self,
        date: &String,
        amount: u32,
        payment_id: u32,
    ) -> Result<usize, rusqlite::Error> {
        let sql = format!(
            "UPDATE {} SET date =?1, amount = ?2 WHERE id = ?3",
            constants::PAYMENTS_TABLE
        );

        self.conn.execute(&sql, params![date, amount, payment_id])
    }

    pub fn delete_payment(&self, payment_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::PAYMENTS_TABLE);

        self.conn.execute(&sql, params![payment_id])
    }
}
