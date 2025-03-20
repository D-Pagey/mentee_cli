use rusqlite::Connection;

use crate::config::Config;

pub fn establish_connection(config: &Config) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(&config.db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}
