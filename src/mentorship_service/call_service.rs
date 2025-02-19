use crate::{constants, error::MenteeError};
use chrono::NaiveDate;
use inquire::{DateSelect, Text};
use rusqlite::{params, Connection, OptionalExtension};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Call {
    pub id: u32,
    pub mentee_id: u32,
    pub date: String,
    pub notes: String,
}

#[derive(Debug)]
pub struct CallWithMenteeName {
    pub call_id: u32,
    pub mentee_name: String,
    pub date: String,
    pub notes: String,
}

pub struct CallService {
    conn: Rc<RefCell<Connection>>,
}

impl CallService {
    // TODO: change error to a CallError
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        Ok(Self { conn })
    }

    pub fn get_all_calls(
        &self,
        name: Option<String>,
    ) -> Result<Vec<CallWithMenteeName>, MenteeError> {
        let mut sql = format!(
            "
            SELECT 
                calls.id AS call_id,
                mentees.name AS mentee_name,
                calls.date,
                calls.notes
            FROM 
                {}
            JOIN 
                {}
            ON
                calls.mentee_id = mentees.id
            ",
            constants::CALLS_TABLE,
            constants::MENTEES_TABLE
        );

        if let Some(name) = name {
            let mentee_id = match self.get_mentee_id(&name)? {
                Some(id) => id,
                None => {
                    // TODO: change this to error not OK
                    println!("No mentee found with the name '{}'.", name);
                    return Ok(vec![]); // Return early with an empty vector
                }
            };

            sql.push_str(format!("WHERE calls.mentee_id = {} ", &mentee_id).as_str());
        }

        sql.push_str("ORDER BY calls.date DESC");

        let binding = self.conn.borrow();
        let mut stmt = binding.prepare(&sql)?;

        let call_iter = stmt.query_map([], |row| {
            Ok(CallWithMenteeName {
                call_id: row.get(0)?,
                mentee_name: row.get(1)?,
                date: row.get(2)?,
                notes: row.get(3)?,
            })
        })?;

        let mut calls: Vec<CallWithMenteeName> = Vec::new();

        for call_result in call_iter {
            calls.push(call_result?)
        }

        Ok(calls)
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

    pub fn add_call(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = match self.get_mentee_id(&name)? {
            Some(id) => id,
            None => return Ok(format!("No mentee found with the name '{}'.", name)),
        };

        let date = DateSelect::new("Enter the date of the call:")
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let notes = Text::new("Enter any notes for the call:")
            .with_placeholder("e.g. Discussed project progress ")
            .prompt()
            .expect("Failed to read notes");

        let result = self.conn.borrow().execute(
            &format!(
                "INSERT INTO {} (mentee_id, date, notes) VALUES (?1, ?2, ?3)",
                constants::CALLS_TABLE
            ),
            (&mentee_id, &date, &notes),
        );

        match result {
            Ok(_) => Ok(format!("Call with {name} on {date} added.")),
            Err(err) => Err(MenteeError::from(err)),
        }
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
                    id: row.get(0)?,
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
            .with_initial_value(&call.notes)
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
