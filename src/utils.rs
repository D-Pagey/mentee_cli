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

// Define a reusable validator for days of the month
// fn validate_day_core(input: &u32) -> Result<Validation, Box<dyn Error + Send + Sync>> {
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
