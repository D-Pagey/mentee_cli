use crate::ColumnOptions;
use crate::{constants, error::MenteeError};

use crate::mentee::{Mentee, Status};
use inquire::{Select, Text};
use rusqlite::{Connection, Result};

pub struct MenteeService {
    conn: Connection,
}

impl MenteeService {
    pub fn new(database_url: &str) -> Result<Self, MenteeError> {
        let conn = Connection::open(database_url)?;

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            calls INTEGER,
            gross INTEGER NOT NULL,
            net INTEGER NOT NULL,
            status TEXT NOT NULL CHECK(status IN ('archived', 'cold', 'warm', 'hot')),
            payment_day INTEGER NOT NULL CHECK(payment_day BETWEEN 1 AND 31))",
            constants::MENTEE_TABLE
        );

        conn.execute(&sql, ())?;

        Ok(MenteeService { conn })
    }

    fn select_status() -> Result<Status, MenteeError> {
        // generate options from enum variants
        let options = Status::variants();
        let selected = Select::new("Select the mentee's status", options).prompt()?;

        Status::from_str(&selected).ok_or_else(|| "Invalid status selected".into())
    }

    pub fn add_mentee(&self) -> Result<Mentee, MenteeError> {
        let name = Text::new("What is their name?").prompt()?;
        let calls = inquire::prompt_u32("How many calls per month do they have?")?;
        let gross = inquire::prompt_u32("What is the gross payment?")?;
        let net = inquire::prompt_u32("What is the net payment?")?;
        let status = MenteeService::select_status()?;
        let payment_day = inquire::prompt_u32("Which day of the month do they pay?")?;

        let mentee = Mentee {
            name,
            calls,
            gross,
            net,
            status,
            payment_day,
        };

        let result = self.conn.execute(
            &format!(
                "INSERT INTO {} (name, calls, gross, net, status, payment_day) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                constants::MENTEE_TABLE
            ),
            (
                &mentee.name,
                &mentee.calls,
                &mentee.gross,
                &mentee.net,
                Status::as_str(&mentee.status),
                &mentee.payment_day,
            ),
        );

        match result {
            Ok(_) => Ok(mentee),
            Err(rusqlite::Error::SqliteFailure(ref err, _)) if err.extended_code == 2067 => {
                Err(MenteeError::UniqueViolation(mentee.name))
            }
            Err(err) => Err(MenteeError::from(err)),
        }
    }

    pub fn delete_mentee(&self, name: String) -> Result<usize, MenteeError> {
        let deleted = self.conn.execute(
            &format!("DELETE FROM {} WHERE name = :name", constants::MENTEE_TABLE),
            &[(":name", &name.to_lowercase())],
        )?;

        Ok(deleted)
    }

    pub fn update_mentee(&self, name: String) -> Result<usize, MenteeError> {
        let calls = Text::new("How many calls per month do they have?").prompt()?;

        let updated = self.conn.execute(
            &format!(
                "UPDATE {} SET calls = ?1 WHERE name = ?2",
                constants::MENTEE_TABLE
            ),
            (&calls, &name),
        )?;

        Ok(updated)
    }

    pub fn get_all_mentees(&self) -> Result<Vec<Mentee>, MenteeError> {
        let sql = format!("SELECT * FROM {}", constants::MENTEE_TABLE);
        let mut stmt = self.conn.prepare(&sql)?;
        let mentee_iter = stmt.query_map([], |row| {
            let status_str: String = row.get(5)?;

            let status = Status::from_str(&status_str).unwrap_or(Status::Warm);

            Ok(Mentee {
                name: row.get(1)?,
                calls: row.get(2)?,
                gross: row.get(3)?,
                net: row.get(4)?,
                status,
                payment_day: row.get(6)?,
            })
        })?;

        let mut mentees: Vec<Mentee> = Vec::new();

        for mentee_result in mentee_iter {
            mentees.push(mentee_result?)
        }

        Ok(mentees)
    }

    pub fn get_mentee_count(&self, count: Option<ColumnOptions>) -> Result<String, MenteeError> {
        let (sql, message) = match count {
            Some(ColumnOptions::Calls) => ("SELECT SUM(calls) FROM mentees", "Number of calls"),
            _ => ("SELECT COUNT(*) FROM mentees", "Number of mentees"),
        };

        let result: i64 = self.conn.query_row(sql, [], |row| row.get(0))?;

        Ok(format!("{}: {}", message, result))
    }
}
