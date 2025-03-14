use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    constants,
    models::mentee::{Mentee, MenteeSummary, MenteeWithCounts, Status},
};

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

    pub fn get_mentee_with_counts(
        &self,
        name: &String,
    ) -> Result<MenteeWithCounts, rusqlite::Error> {
        let sql = format!(
            "
            SELECT 
                mentees.*,
                COALESCE(COUNT(DISTINCT calls.id), 0) AS call_count, 
                COALESCE(COUNT(DISTINCT payments.id), 0) AS payment_count,
                COALESCE(COUNT(DISTINCT videos.id), 0) AS video_count,
                (mentees.calls * COALESCE(COUNT(DISTINCT payments.id), 0)) - COALESCE(COUNT(DISTINCT calls.id), 0) AS remaining_calls
            FROM 
                {}
            LEFT JOIN
                {} ON calls.mentee_id = mentees.id
            LEFT JOIN 
                {} ON payments.mentee_id = mentees.id
            LEFT JOIN 
                {} ON videos.mentee_id = mentees.id
            WHERE 
                name = ?
            GROUP BY
                mentees.id
            ",
            constants::MENTEES_TABLE,
            constants::CALLS_TABLE,
            constants::PAYMENTS_TABLE,
            constants::VIDEOS_TABLE
        );

        self.conn.query_row(&sql, params![name], |row| {
            let status_str: String = row.get(5)?;
            let status = Status::from_str(&status_str).unwrap_or(Status::Warm);

            Ok(MenteeWithCounts {
                mentee: Mentee {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    calls: row.get(2)?,
                    gross: row.get(3)?,
                    net: row.get(4)?,
                    status,
                    payment_day: row.get(6)?,
                    notes: row.get(7)?,
                },
                call_count: row.get(8)?,
                payment_count: row.get(9)?,
                video_count: row.get(10)?,
                remaining_calls: row.get(11)?,
            })
        })
    }

    pub fn get_all_mentees(&self, show_all: bool) -> Result<Vec<MenteeSummary>, rusqlite::Error> {
        let mut sql = format!(
            "
            SELECT 
                mentees.id,
                mentees.name,
                mentees.calls,
                (mentees.calls * COALESCE(COUNT(DISTINCT payments.id), 0)) - COALESCE(COUNT(DISTINCT calls.id), 0) AS remaining_calls,
                mentees.status,
                mentees.notes
            FROM 
                {}
            LEFT JOIN
                {} ON calls.mentee_id = mentees.id
            LEFT JOIN 
                {} ON payments.mentee_id = mentees.id
            ",
            constants::MENTEES_TABLE,
            constants::CALLS_TABLE,
            constants::PAYMENTS_TABLE
        );

        if !show_all {
            sql = format!("{} WHERE status != 'archived'", sql)
        }

        sql = format!(
            "{} 
            GROUP BY
                mentees.id
            ORDER BY 
                CASE status 
                    WHEN 'hot' THEN 1
                    WHEN 'warm' THEN 2
                    WHEN 'cold' THEN 3
                    ELSE 4
                END
            ",
            sql
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let mentee_iter = stmt.query_map([], |row| {
            let status_str: String = row.get(4)?;

            let status = Status::from_str(&status_str).unwrap_or(Status::Warm);

            Ok(MenteeSummary {
                name: row.get(1)?,
                calls_per_month: row.get(2)?,
                remaining_calls: row.get(3)?,
                status,
                notes: row.get(5)?,
            })
        })?;

        let mut mentees: Vec<MenteeSummary> = Vec::new();

        for mentee_result in mentee_iter {
            mentees.push(mentee_result?)
        }

        Ok(mentees)
    }

    pub fn delete_mentee_by_id(&self, id: i64) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::MENTEES_TABLE);

        self.conn.execute(&sql, params![id])
    }
}
