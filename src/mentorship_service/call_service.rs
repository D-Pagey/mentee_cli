use crate::{constants, error::MenteeError};
use inquire::{DateSelect, Text};
use rusqlite::{Connection, OptionalExtension};
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
        let calls_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
            constants::CALLS_TABLE,
            constants::MENTEE_TABLE
        );

        conn.borrow().execute(&calls_sql, ())?;

        Ok(CallService { conn })
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
            ORDER BY 
                calls.date DESC
            ",
            constants::CALLS_TABLE,
            constants::MENTEE_TABLE
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

            sql.push_str(format!("WHERE calls.mentee_id = {}", &mentee_id).as_str());
        }

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
            constants::MENTEE_TABLE,
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

    pub fn delete_call(&self, call_id: u32) -> Result<String, MenteeError> {
        let deleted = self.conn.borrow().execute(
            &format!("DELETE FROM {} WHERE id = :call_id", constants::CALLS_TABLE),
            &[(":call_id", &call_id)],
        )?;

        if deleted > 0 {
            Ok(format!("Deleted call with id of {}", call_id.to_string()))
        } else {
            Ok(format!(
                "Could not find a call with id of {}",
                call_id.to_string()
            ))
        }
    }
}
