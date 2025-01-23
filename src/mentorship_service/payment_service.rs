use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::Connection;

use crate::constants;
use crate::error::MenteeError;

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
            amount TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
            constants::PAYMENT_TABLE,
            constants::MENTEE_TABLE
        );

        conn.borrow().execute(&sql, ())?;

        Ok(Self { conn })
    }
}
