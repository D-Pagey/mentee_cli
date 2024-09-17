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
    ValidationError(String),
}

impl fmt::Display for MenteeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenteeError::DatabaseError(err) => write!(f, "Database error: {}", err),
            MenteeError::IOError(err) => write!(f, "IO error: {}", err),
            MenteeError::InquireError(err) => write!(f, "Inquire error: {}", err),
            MenteeError::NotFound(resource) => write!(f, "{} not found", resource),
            MenteeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MenteeError::ValidationError(msg) => write!(f, "Invalid input: {}", msg),
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

impl From<&str> for MenteeError {
    fn from(message: &str) -> MenteeError {
        MenteeError::ValidationError(message.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inquire::InquireError;
    use rusqlite::Error as RusqliteError;
    use std::io::{Error as IoError, ErrorKind};

    #[test]
    fn test_database_error_display() {
        let db_err = RusqliteError::InvalidQuery;
        let mentee_error = MenteeError::DatabaseError(db_err);
        assert_eq!(
            format!("{}", mentee_error),
            "Database error: Query is not read-only"
        );
    }

    #[test]
    fn test_io_error_display() {
        let io_err = IoError::new(ErrorKind::NotFound, "file not found");
        let mentee_error = MenteeError::IOError(io_err);
        assert_eq!(format!("{}", mentee_error), "IO error: file not found");
    }

    #[test]
    fn test_inquire_error_display() {
        let inquire_err = InquireError::Custom("custom error".to_string().into());
        let mentee_error = MenteeError::InquireError(inquire_err);
        assert_eq!(
            format!("{}", mentee_error),
            "Inquire error: User-provided error: custom error"
        );
    }

    #[test]
    fn test_inquire_error_from_inquire() {
        let inquire_err = InquireError::Custom("custom error".to_string().into());
        let mentee_error: MenteeError = inquire_err.into();
        assert!(matches!(mentee_error, MenteeError::InquireError(_)));
    }

    #[test]
    fn test_not_found_display() {
        let mentee_error = MenteeError::NotFound("Mentee".to_string());
        assert_eq!(format!("{}", mentee_error), "Mentee not found");
    }

    #[test]
    fn test_invalid_input_display() {
        let mentee_error = MenteeError::InvalidInput("Invalid name".to_string());
        assert_eq!(format!("{}", mentee_error), "Invalid input: Invalid name");
    }

    #[test]
    fn test_unique_violation_display() {
        let mentee_error = MenteeError::UniqueViolation("John Doe".to_string());
        assert_eq!(
            format!("{}", mentee_error),
            "Mentee with name 'John Doe' already exists."
        );
    }

    #[test]
    fn test_validation_error_from_str() {
        let mentee_error: MenteeError = "Validation error".into();
        assert_eq!(
            format!("{}", mentee_error),
            "Invalid input: Validation error"
        );
    }

    #[test]
    fn test_database_error_from_rusqlite() {
        let db_err = RusqliteError::InvalidQuery;
        let mentee_error: MenteeError = db_err.into();
        assert!(matches!(mentee_error, MenteeError::DatabaseError(_)));
    }

    #[test]
    fn test_io_error_from_io() {
        let io_err = IoError::new(ErrorKind::NotFound, "file not found");
        let mentee_error: MenteeError = io_err.into();
        assert!(matches!(mentee_error, MenteeError::IOError(_)));
    }
}
