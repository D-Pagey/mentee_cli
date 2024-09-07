use crate::ColumnOptions;
use crate::{constants, error::MenteeError};

use crate::mentee::Mentee;
use inquire::Text;
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
            calls INTEGER
            )",
            constants::MENTEE_TABLE
        );

        conn.execute(&sql, ())?;

        Ok(MenteeService { conn })
    }

    // TODO: add validator to parse to number then check max calls
    // how to use the parsing_u32
    pub fn add_mentee(&self) -> Result<Mentee, MenteeError> {
        let name = Text::new("What is their name?").prompt()?;
        let calls = inquire::prompt_u32("How many calls per month do they have?")?;

        let mentee = Mentee { name, calls };

        let result = self.conn.execute(
            &format!(
                "INSERT INTO {} (name, calls) VALUES (?1, ?2)",
                constants::MENTEE_TABLE
            ),
            (&mentee.name, &mentee.calls),
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
        let sql = format!("SELECT name, calls FROM {}", constants::MENTEE_TABLE);
        let mut stmt = self.conn.prepare(&sql)?;
        let mentee_iter = stmt.query_map([], |row| {
            Ok(Mentee {
                name: row.get(0)?,
                calls: row.get(1)?,
            })
        })?;

        let mut mentees: Vec<Mentee> = Vec::new();

        for mentee_result in mentee_iter {
            mentees.push(mentee_result?)
        }

        Ok(mentees)
    }

    pub fn get_mentee_count(&self, count: ColumnOptions) -> Result<String, MenteeError> {
        let (sql, message) = match count {
            ColumnOptions::Name => ("SELECT COUNT(*) FROM mentees", "Number of mentees"),
            ColumnOptions::Calls => ("SELECT SUM(calls) FROM mentees", "Number of calls"),
        };

        let result: i64 = self.conn.query_row(sql, [], |row| row.get(0))?;

        Ok(format!("{}: {}", message, result))
    }
}
