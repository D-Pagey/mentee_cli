use rusqlite::{params, Connection};

use crate::{
    constants,
    models::payment::{Payment, PaymentWithMenteeName},
};

pub struct PaymentRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PaymentRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn add_payment(
        &self,
        mentee_id: u32,
        date: String,
        amount: u32,
    ) -> Result<usize, rusqlite::Error> {
        let sql = format!(
            "INSERT INTO {} (mentee_id, date, amount) VALUES (?1, ?2, ?3)",
            constants::PAYMENTS_TABLE
        );

        self.conn.execute(&sql, params![mentee_id, date, amount])
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

    pub fn get_all_payments(
        &self,
        mentee_id: Option<i64>,
    ) -> Result<Vec<PaymentWithMenteeName>, rusqlite::Error> {
        let mut sql = format!(
            "SELECT 
                payments.id AS payment_id,
                mentees.name AS mentee_name,
                payments.date,
                payments.amount
            FROM 
                {}
            JOIN
                {}
            ON
                payments.mentee_id = mentees.id            
            ",
            constants::PAYMENTS_TABLE,
            constants::MENTEES_TABLE
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let id_storage;

        if let Some(id) = mentee_id {
            sql.push_str(" WHERE payments.mentee_id = ?1");
            id_storage = id;
            params.push(&id_storage);
        }

        sql.push_str(" ORDER BY payments.date DESC");

        let mut stmt = self.conn.prepare(&sql)?;
        let payment_iter = stmt.query_map(&params[..], |row| {
            Ok(PaymentWithMenteeName {
                id: row.get(0)?,
                mentee_name: row.get(1)?,
                date: row.get(2)?,
                amount: row.get(3)?,
            })
        })?;

        let mut payments = Vec::new();
        for payment in payment_iter {
            payments.push(payment?);
        }

        Ok(payments)
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
