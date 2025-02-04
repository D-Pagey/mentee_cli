use std::cell::RefCell;
use std::rc::Rc;

use chrono::NaiveDate;
use inquire::{CustomType, DateSelect};
use rusqlite::{params, Connection, OptionalExtension};

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

    pub fn get_payments(self, name: Option<String>) -> Result<Vec<Payment>, MenteeError> {
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

        if let Some(name) = name {
            let mentee_id = match self.get_mentee_id(&name)? {
                Some(id) => id,
                None => {
                    // TODO: change this to error not OK // or should it be error?
                    println!("No mentee found with the name '{}'.", name);
                    return Ok(vec![]); // Return early with an empty vector
                }
            };

            sql.push_str(format!("WHERE payments.mentee_id = {} ", &mentee_id).as_str());
        }

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

    // TODO: deduplicate this
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

    fn parse_date_from_db(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
    }

    pub fn update_payment(self, payment_id: u32) -> Result<String, MenteeError> {
        let get_sql = format!("SELECT * FROM {} WHERE id = ?1", constants::PAYMENTS_TABLE);

        let result = self
            .conn
            .borrow()
            .query_row(&get_sql, params![payment_id], |row| {
                Ok(Payment {
                    id: row.get(0)?,
                    mentee_id: row.get(1)?,
                    date: row.get(2)?,
                    amount: row.get(3)?,
                    mentee_name: None,
                })
            });

        let payment = match result {
            Ok(payment) => payment,
            _ => return Ok(format!("Can't find a payment with id of {}", payment_id)),
        };

        // TODO: deal with this
        let parsed = PaymentService::parse_date_from_db(&payment.date).unwrap();

        let date = DateSelect::new("Enter the date of the payment:")
            .with_default(parsed)
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let amount: u32 = CustomType::new("How much?")
            .with_starting_input(&payment.amount.to_string())
            .prompt()
            .expect("Failed to read amount");

        let update_sql = format!(
            "UPDATE {} SET date = ?1, amount = ?2 WHERE id = ?3",
            constants::PAYMENTS_TABLE
        );

        let result = self
            .conn
            .borrow()
            .execute(&update_sql, params![date, amount, payment_id])?;

        Ok(format!("{result} payment record updated"))
    }

    pub fn delete_payment(self, id: u32) -> Result<String, MenteeError> {
        let sql = format!(
            "DELETE FROM {} WHERE id = :payment_id",
            constants::PAYMENTS_TABLE
        );

        let deleted = self.conn.borrow().execute(&sql, &[(":payment_id", &id)])?;

        if deleted > 0 {
            Ok(format!("Payment with id = {id} deleted."))
        } else {
            Ok(format!("Couldn't find payment with id of {id}"))
        }
    }
}
