use rusqlite::{params, Connection, OptionalExtension};

use crate::{constants, models::mentee::Mentee};

pub struct MenteeRepository<'a> {
    conn: &'a Connection,
}

impl<'a> MenteeRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Fetches a mentee's ID by name
    pub fn get_mentee_id(&self, name: &str) -> Result<Option<i64>, rusqlite::Error> {
        let sql = format!(
            "SELECT id FROM {} WHERE name = ?1 LIMIT 1",
            constants::MENTEES_TABLE
        );

        self.conn
            .query_row(&sql, params![name], |row| row.get(0))
            .optional()
    }

    pub fn add_mentee(&self, mentee: Mentee) -> Result<usize, rusqlite::Error> {
        let sql = format!(
            "INSERT INTO {} (name, calls, gross, net, status, payment_day, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
            constants::MENTEES_TABLE);

        self.conn.execute(
            &sql,
            params![
                mentee.name,
                mentee.calls,
                mentee.gross,
                mentee.net,
                mentee.status.as_str(),
                mentee.payment_day,
                mentee.notes
            ],
        )
    }

    pub fn delete_mentee_by_id(&self, id: i64) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::MENTEES_TABLE);

        self.conn.execute(&sql, params![id])
    }
}
