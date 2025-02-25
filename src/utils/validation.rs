use inquire::validator::Validation;
use std::error::Error;

use chrono::{NaiveDate, ParseError};

pub fn parse_date_from_db(date_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

pub fn validate_name_core(s: &str) -> Result<(), String> {
    if s.trim().is_empty() {
        Err("Name cannot be empty or just whitespace.".to_string())
    } else if s.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        Ok(())
    } else {
        Err("Name can only contain letters and spaces.".to_string())
    }
}

pub fn inquire_validate_name(s: &str) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    match validate_name_core(s) {
        Ok(()) => Ok(Validation::Valid),
        Err(err) => Err(err.into()), // Convert String into Box<dyn Error>
    }
}

pub fn clap_validate_name(s: &str) -> Result<String, String> {
    match validate_name_core(s) {
        Ok(()) => Ok(s.to_string()),
        Err(err) => Err(err),
    }
}

fn validate_day_core(input: &u32) -> Result<(), String> {
    if *input >= 1 && *input <= 31 {
        Ok(())
    } else {
        Err("The day must be between 1 and 31.".to_string())
    }
}

pub fn inquire_validate_day(input: &u32) -> Result<Validation, Box<dyn Error + Send + Sync>> {
    match validate_day_core(input) {
        Ok(()) => Ok(Validation::Valid),
        Err(err) => Err(err.into()), // Convert String into Box<dyn Error>
    }
}

pub fn clap_validate_day(input: &str) -> Result<String, String> {
    let parsed = input.parse::<u32>().expect("Not a valid number");

    match validate_day_core(&parsed) {
        Ok(()) => Ok(input.to_string()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_for_valid_name() {
        let result = validate_name_core("dan");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn error_for_empty_name() {
        let result = validate_name_core("     ");
        assert_eq!(
            result,
            Err(String::from("Name cannot be empty or just whitespace."))
        );
    }

    #[test]
    fn error_for_invalid_name() {
        let result = validate_name_core("d#n");
        assert_eq!(
            result,
            Err(String::from("Name can only contain letters and spaces."))
        );
    }

    #[test]
    fn inquire_valid_name() {
        let result = inquire_validate_name("dan");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Validation::Valid);
    }

    #[test]
    fn inquire_invalid_name() {
        let result = inquire_validate_name("d#n");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Name can only contain letters and spaces."
        );
    }

    #[test]
    fn clap_valid_name() {
        let name = "dan";
        let result = clap_validate_name(name);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), name)
    }

    #[test]
    fn clap_invalid_name() {
        let result = clap_validate_name("d#n");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Name can only contain letters and spaces."
        )
    }

    #[test]
    fn valid_day() {
        let result = validate_day_core(&2);
        assert!(result.is_ok());
        assert_eq!(result, Ok(()))
    }

    #[test]
    fn invalid_day() {
        let result = validate_day_core(&60);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The day must be between 1 and 31."
        )
    }

    #[test]
    fn inquire_valid_day() {
        let result = inquire_validate_day(&20);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Validation::Valid)
    }

    #[test]
    fn inquire_invalid_day() {
        let result = inquire_validate_day(&70);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The day must be between 1 and 31."
        )
    }

    #[test]
    fn clap_valid_day() {
        let result = clap_validate_day("20");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "20")
    }

    #[test]
    fn clap_invalid_day() {
        let result = clap_validate_day("70");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The day must be between 1 and 31."
        )
    }
}
