use std::cell::RefCell;
use std::rc::Rc;

use inquire::{CustomType, DateSelect};
use rusqlite::{Connection, OptionalExtension};

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
        Ok(Self { conn })
    }

    fn get_mentee_id(&self, name: &str) -> Result<Option<i64>, rusqlite::Error> {
        let sql = format!(
            "SELECT id FROM {} WHERE name = ? LIMIT 1",
            constants::MENTEES_TABLE,
        );

        self.conn
            .borrow()
            .query_row(&sql, &[name], |row| row.get(0))
            .optional()
    }

    pub fn add_payment(self, name: String) -> Result<String, MenteeError> {
        let mentee_id = match self.get_mentee_id(&name)? {
            Some(id) => id,
            None => return Ok(format!("No mentee found with the name '{}'.", name)),
        };

        let date = DateSelect::new("Enter the date of the payment:")
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let amount: u32 = CustomType::new("Enter the payment amount:")
            .with_placeholder("e.g., 100")
            .prompt()
            .expect("Failed to read amount");

        let result = self.conn.borrow().execute(
            &format!(
                "INSERT INTO {} (mentee_id, date, amount) VALUES (?1, ?2, ?3)",
                constants::PAYMENTS_TABLE
            ),
            (&mentee_id, &date, &amount),
        );

        match result {
            Ok(_) => Ok(format!("Payment with {name} on {date} added.")),
            Err(err) => Err(MenteeError::from(err)),
        }
    }
}
