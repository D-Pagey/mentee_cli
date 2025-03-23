use rusqlite::{params, Connection, OptionalExtension};

use crate::{
    constants,
    models::mentee::{Mentee, MenteeSummary, MenteeWithCounts, Status},
    CountOptions, UpdateMentee,
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
                (mentees.calls * COALESCE(COUNT(DISTINCT payments.id), 0)) - COALESCE(SUM(CASE WHEN calls.free_call = 0 THEN 1 ELSE 0 END), 0) AS remaining_calls
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

    pub fn get_mentee_count(
        &self,
        count_type: Option<CountOptions>,
    ) -> Result<i64, rusqlite::Error> {
        let sql = match count_type {
            Some(CountOptions::Calls) => "SELECT SUM(calls) FROM mentees",
            Some(CountOptions::Gross) => "SELECT SUM(gross) FROM mentees",
            Some(CountOptions::Net) => "SELECT SUM(net) FROM mentees",
            Some(CountOptions::NetPerCall) => {
                "SELECT CAST(AVG(net_per_call) AS INTEGER) AS average_net_per_call
                    FROM (
                        SELECT CASE 
                            WHEN calls > 0 THEN net / calls 
                            ELSE net 
                            END AS net_per_call
                    FROM mentees
                )"
            }
            _ => "SELECT COUNT(*) FROM mentees",
        };

        let sql = format!("{} WHERE status != 'archived'", sql);
        self.conn.query_row(&sql, [], |row| row.get(0))
    }

    pub fn update_mentee(&self, update_args: &UpdateMentee) -> Result<usize, rusqlite::Error> {
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(new_name) = &update_args.new_name {
            updates.push("name = ?");
            params.push(Box::new(new_name));
        }

        if let Some(calls) = update_args.calls {
            updates.push("calls = ?");
            params.push(Box::new(calls));
        }

        if let Some(gross) = update_args.gross {
            updates.push("gross = ?");
            params.push(Box::new(gross));
        }

        if let Some(net) = update_args.net {
            updates.push("net = ?");
            params.push(Box::new(net));
        }

        if let Some(status) = update_args.status.as_ref() {
            updates.push("status = ?");
            params.push(Box::new(status.as_str()));
        }

        if let Some(payment_day) = update_args.payment_day {
            updates.push("payment_day = ?");
            params.push(Box::new(payment_day));
        }

        if let Some(notes) = update_args.notes.as_ref() {
            updates.push("notes = ?");
            params.push(Box::new(notes));
        }

        if updates.is_empty() {
            return Ok(0); // No updates to make
        }

        // Join updates into a single SQL statement
        let updates_str = updates.join(", ");

        let sql = format!(
            "UPDATE {} SET {} WHERE name = ?",
            crate::constants::MENTEES_TABLE,
            updates_str
        );

        let mut params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|s| s.as_ref()).collect();

        // Append mentee name to params (for WHERE clause)
        params_refs.push(&update_args.name);

        self.conn.execute(&sql, params_refs.as_slice())
    }
}
