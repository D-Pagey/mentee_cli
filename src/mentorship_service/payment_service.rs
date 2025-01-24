use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::Connection;

use crate::constants;
use crate::error::MenteeError;

pub struct Payment {
    pub id: u32,
    pub mentee_id: u32,
    pub date: String,
    pub amount: u32,
    pub mentee_name: Option<String>,
}

pub struct PaymentService {
    conn: Rc<RefCell<Connection>>,
}

impl PaymentService {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
            constants::PAYMENTS_TABLE,
            constants::MENTEES_TABLE
        );

        conn.borrow().execute(&sql, ())?;

        Ok(Self { conn })
    }

    pub fn get_payments(self) -> Result<Vec<Payment>, MenteeError> {
        let mut sql = format!(
            "SELECT 
                payments.id AS payment_id,
                mentees.name AS mentee_name,
                payments.mentee_id,
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

        sql.push_str("ORDER BY payments.date DESC");

        let binding = self.conn.borrow();
        let mut stmt = binding.prepare(&sql)?;

        let payment_iter = stmt.query_map([], |row| {
            Ok(Payment {
                id: row.get(0)?,
                mentee_name: row.get(1)?,
                mentee_id: row.get(2)?,
                date: row.get(3)?,
                amount: row.get(4)?,
            })
        })?;

        let mut payments: Vec<Payment> = Vec::new();

        for payment_result in payment_iter {
            payments.push(payment_result?)
        }

        Ok(payments)
    }
}
