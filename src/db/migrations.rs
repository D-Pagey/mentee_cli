use rusqlite::Connection;

use crate::constants;

pub fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    let calls_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
        constants::CALLS_TABLE,
        constants::MENTEES_TABLE
    );

    conn.execute(&calls_sql, [])?;

    let videos_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            length INTEGER NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
        constants::VIDEOS_TABLE,
        constants::MENTEES_TABLE
    );

    conn.execute(&videos_sql, [])?;

    Ok(())
}
