use rusqlite::{Connection, Result};
use std::process;

fn main() -> Result<()> {
    let conn = Connection::open("mentees.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mentee (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        calls INTEGER
    )",
        (),
    )?;

    if let Err(e) = mentees::run(conn) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
