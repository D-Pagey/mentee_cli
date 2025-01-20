use crate::{constants, error::MenteeError};
use dirs::home_dir;
use rusqlite::{Connection, OptionalExtension};

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

#[derive(Debug)]
pub struct CallWithMenteeName {
    pub call_id: u32,
    pub mentee_name: String,
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

    pub fn get_all_calls(&self) -> Result<Vec<CallWithMenteeName>, MenteeError> {
        let sql = format!(
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
            constants::MENTEE_TABLE
        );

        let mut stmt = self.conn.prepare(&sql)?;

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
            .query_row(&sql, &[name], |row| row.get(0))
            .optional()
    }

    pub fn add_call(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = match self.get_mentee_id(&name)? {
            Some(id) => id,
            None => return Ok(format!("No mentee found with the name '{}'.", name)),
        };

        Ok(mentee_id.to_string())

        // let call = Call {
        //     id: 1,
        //     mentee_id: 1,
        //     date: "1st January 2025".to_string(),
        //     notes: "Long ass call".to_string(),
        // };
        //
        // let result = self.conn.execute(
        //     &format!(
        //         "INSERT INTO {} (mentee_id, date, notes) VALUES (?1, ?2, ?3)",
        //         constants::CALLS_TABLE
        //     ),
        //     (&call.mentee_id, &call.date, &call.notes),
        // );
        //
        // match result {
        //     Ok(_) => Ok(call),
        //     Err(rusqlite::Error::SqliteFailure(ref err, _)) if err.extended_code == 2067 => {
        //         Err(MenteeError::UniqueViolation(call.date))
        //     }
        //     Err(err) => Err(MenteeError::from(err)),
        // }
    }
}
