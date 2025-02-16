use crate::constants;
use rusqlite::Connection;

pub struct CallRepository<'a> {
    conn: &'a Connection,
}

impl<'a> CallRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Delete a call by call id
    /// TODO: add return type
    pub fn delete_call(&self, call_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = :call_id", constants::CALLS_TABLE);

        self.conn.execute(&sql, &[(":call_id", &call_id)])
    }
}
