use std::path::PathBuf;

use dirs::home_dir;

use crate::error::MenteeError;

pub struct Config {
    pub db_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self, MenteeError> {
        let mut db_path = home_dir().ok_or(MenteeError::HomeDirNotFound)?;
        db_path.push(".mentees"); // Directory to store db
        std::fs::create_dir_all(&db_path)?; // Ensure directory exists

        if cfg!(debug_assertions) {
            // Dev database path
            db_path.push("mentees_dev.db");
        } else {
            // Production database path
            db_path.push("mentees.db");
        }

        Ok(Self { db_path })
    }
}
