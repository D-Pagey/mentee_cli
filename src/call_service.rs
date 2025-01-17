use crate::{constants, error::MenteeError};
use dirs::home_dir;
use rusqlite::Connection;

pub struct CallService {
    conn: Connection,
}

#[derive(Debug)]
pub struct Call {
    pub id: u32,
    pub mentee_id: u32,
    pub date: String,
    pub notes: String,
}

impl CallService {
    // TODO: change error to a CallError
    pub fn new() -> Result<Self, MenteeError> {
        let mut db_path = home_dir().ok_or(MenteeError::HomeDirNotFound)?;

        db_path.push(".mentees"); // directory to store db
        std::fs::create_dir_all(&db_path)?; // ensure directory exists
        db_path.push("mentees.db"); // database file name

        let conn = Connection::open(db_path)?;

        let calls_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
            constants::CALLS_TABLE,
            constants::MENTEE_TABLE
        );

        conn.execute(&calls_sql, ())?;

        Ok(CallService { conn })
    }

    pub fn get_all_calls(&self) -> Result<Vec<Call>, MenteeError> {
        let sql = format!("SELECT * FROM {}", constants::CALLS_TABLE);
        let mut stmt = self.conn.prepare(&sql)?;

        let call_iter = stmt.query_map([], |row| {
            Ok(Call {
                id: row.get(0)?,
                mentee_id: row.get(1)?,
                date: row.get(2)?,
                notes: row.get(3)?,
            })
        })?;

        let mut calls: Vec<Call> = Vec::new();

        for call_result in call_iter {
            calls.push(call_result?)
        }

        Ok(calls)
    }

    pub fn add_call(&self) -> Result<Call, MenteeError> {
        let call = Call {
            id: 1,
            mentee_id: 1,
            date: "1st January 2025".to_string(),
            notes: "Long ass call".to_string(),
        };

        let result = self.conn.execute(
            &format!(
                "INSERT INTO {} (mentee_id, date, notes) VALUES (?1, ?2, ?3)",
                constants::CALLS_TABLE
            ),
            (&call.mentee_id, &call.date, &call.notes),
        );

        match result {
            Ok(_) => Ok(call),
            Err(rusqlite::Error::SqliteFailure(ref err, _)) if err.extended_code == 2067 => {
                Err(MenteeError::UniqueViolation(call.date))
            }
            Err(err) => Err(MenteeError::from(err)),
        }
    }
}
