use rusqlite::Connection;

use crate::{config::Config, error::MenteeError};

pub fn establish_connection(config: &Config) -> Result<Connection, MenteeError> {
    Connection::open(&config.db_path).map_err(MenteeError::DatabaseError)
}
