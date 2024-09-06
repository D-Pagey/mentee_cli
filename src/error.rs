use rusqlite;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum MenteeError {
    DatabaseError(rusqlite::Error),
    IOError(io::Error),
    InquireError(inquire::InquireError),
    NotFound(String),
    InvalidInput(String),
    UniqueViolation(String),
}

impl fmt::Display for MenteeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenteeError::DatabaseError(err) => write!(f, "Database error: {}", err),
            MenteeError::IOError(err) => write!(f, "IO error: {}", err),
            MenteeError::InquireError(err) => write!(f, "Inquire error: {}", err),
            MenteeError::NotFound(resource) => write!(f, "{} not found", resource),
            MenteeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MenteeError::UniqueViolation(name) => {
                write!(f, "Mentee with name '{}' already exists.", name)
            }
        }
    }
}

impl From<rusqlite::Error> for MenteeError {
    fn from(err: rusqlite::Error) -> MenteeError {
        MenteeError::DatabaseError(err)
    }
}

impl From<io::Error> for MenteeError {
    fn from(err: io::Error) -> MenteeError {
        MenteeError::IOError(err)
    }
}

impl From<inquire::InquireError> for MenteeError {
    fn from(err: inquire::InquireError) -> MenteeError {
        MenteeError::InquireError(err)
    }
}
