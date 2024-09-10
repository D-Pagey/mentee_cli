use inquire::validator::Validation;
use std::error::Error;

pub fn validate_name_core(s: &str) -> Result<(), String> {
    if s.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
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
