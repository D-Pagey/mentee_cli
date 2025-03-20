use rusqlite::Connection;

use crate::constants;

pub fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    let mentees_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            calls INTEGER,
            gross INTEGER NOT NULL,
            net INTEGER NOT NULL,
            status TEXT NOT NULL CHECK(status IN ('archived', 'cold', 'warm', 'hot')),
            payment_day INTEGER NOT NULL CHECK(payment_day BETWEEN 1 AND 31),
            notes TEXT)",
        constants::MENTEES_TABLE
    );

    conn.execute(&mentees_sql, [])?;

    let calls_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id) ON DELETE CASCADE)",
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
            FOREIGN KEY (mentee_id) REFERENCES {} (id) ON DELETE CASCADE)",
        constants::VIDEOS_TABLE,
        constants::MENTEES_TABLE
    );

    conn.execute(&videos_sql, [])?;

    let payments_sql = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (mentee_id) REFERENCES {} (id) ON DELETE CASCADE)",
        constants::PAYMENTS_TABLE,
        constants::MENTEES_TABLE
    );

    conn.execute(&payments_sql, [])?;

    Ok(())
}
