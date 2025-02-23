use rusqlite::{params, Connection, OptionalExtension};

use crate::constants;

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

    // TODO: cascade deletes
    pub fn delete_mentee_by_id(&self, id: String) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::MENTEES_TABLE);

        self.conn.execute(&sql, params![id])
    }
}
