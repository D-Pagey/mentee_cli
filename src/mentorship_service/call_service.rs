use crate::models::call::Call;
use crate::{constants, error::MenteeError};
use chrono::NaiveDate;
use inquire::{DateSelect, Text};
use rusqlite::{params, Connection};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CallService {
    conn: Rc<RefCell<Connection>>,
}

impl CallService {
    // TODO: change error to a CallError
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        Ok(Self { conn })
    }

    fn parse_date_from_db(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
    }

    pub fn update_call(self, call_id: u32) -> Result<String, MenteeError> {
        let get_sql = format!("SELECT * FROM {} WHERE id = ?1", constants::CALLS_TABLE);

        let result = self
            .conn
            .borrow()
            .query_row(&get_sql, params![call_id], |row| {
                Ok(Call {
                    call_id: row.get(0)?,
                    mentee_id: row.get(1)?,
                    date: row.get(2)?,
                    notes: row.get(3)?,
                })
            });

        let call = match result {
            Ok(call) => call,
            _ => return Ok(format!("Can't find a call with id of {}", call_id)),
        };

        // TODO: deal with this
        let parsed = CallService::parse_date_from_db(&call.date).unwrap();

        let date = DateSelect::new("Enter the date of the call:")
            .with_default(parsed)
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let notes = Text::new("Enter any notes for the call:")
            .with_placeholder("e.g. Discussed project progress ")
            .with_initial_value(call.notes.as_deref().unwrap_or(""))
            .prompt()
            .expect("Failed to read notes");

        let update_sql = format!(
            "UPDATE {} SET date = ?1, notes = ?2 WHERE id = ?3",
            constants::CALLS_TABLE
        );

        let result = self
            .conn
            .borrow()
            .execute(&update_sql, params![date, notes, call_id])?;

        Ok(format!("{result} call record updated"))
    }
}
