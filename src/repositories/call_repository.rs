use crate::{constants, models::call::CallWithMenteeName};
use rusqlite::Connection;

pub struct CallRepository<'a> {
    conn: &'a Connection,
}

impl<'a> CallRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_calls(
        &self,
        mentee_id: Option<i64>,
    ) -> Result<Vec<CallWithMenteeName>, rusqlite::Error> {
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

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let id_storage;

        if let Some(id) = mentee_id {
            sql.push_str(" WHERE calls.mentee_id = ?1");
            id_storage = id;
            params.push(&id_storage);
        }

        sql.push_str(" ORDER BY calls.date DESC");

        let mut stmt = self.conn.prepare(&sql)?;
        let call_iter = stmt.query_map(&params[..], |row| {
            Ok(CallWithMenteeName {
                call_id: row.get(0)?,
                mentee_name: row.get(1)?,
                date: row.get(2)?,
                notes: row.get(3)?,
            })
        })?;

        let mut calls = Vec::new();
        for call in call_iter {
            calls.push(call?);
        }

        Ok(calls)
    }
    /// Delete a call by call id
    pub fn delete_call(&self, call_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = :call_id", constants::CALLS_TABLE);

        self.conn.execute(&sql, &[(":call_id", &call_id)])
    }
}
