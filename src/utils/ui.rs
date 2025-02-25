use inquire::Select;

use crate::{error::MenteeError, models::mentee::Status};

pub fn select_status() -> Result<Status, MenteeError> {
    // generate options from enum variants
    let options = Status::variants();
    let selected = Select::new("Select the mentee's status", options).prompt()?;

    Status::from_str(&selected).ok_or_else(|| "Invalid status selected".into())
}
