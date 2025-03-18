use inquire::{CustomType, Text};
use rusqlite::Connection;

use crate::{
    error::MenteeError,
    models::mentee::{Mentee, MenteeSummary, MenteeWithCounts},
    repositories::MenteeRepository,
    utils::{
        ui::select_status,
        validation::{inquire_validate_day, inquire_validate_name},
    },
    CountOptions, UpdateMentee,
};

pub struct MenteeService<'a> {
    mentee_repo: MenteeRepository<'a>,
}

impl<'a> MenteeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            mentee_repo: MenteeRepository::new(conn),
        }
    }

    pub fn add_mentee(&self) -> Result<String, MenteeError> {
        let name = Text::new("What is their name?")
            .with_validator(inquire_validate_name)
            .prompt()?
            .to_lowercase();

        let calls = inquire::prompt_u32("How many calls per month do they have?")?;
        let gross = inquire::prompt_u32("What is the gross payment?")?;
        let net = inquire::prompt_u32("What is the net payment?")?;
        let status = select_status()?;
        let payment_day: u32 = CustomType::new("Which day of the month do they pay?")
            .with_validator(inquire_validate_day)
            .prompt()?;
        let notes = Text::new("Any notes about them?").prompt()?;

        let mentee = Mentee {
            id: 0,
            name: name.clone(),
            calls,
            gross,
            net,
            status,
            payment_day,
            notes: Some(notes),
        };

        let result = self.mentee_repo.add_mentee(mentee);

        match result {
            Ok(_) => Ok(name),
            Err(rusqlite::Error::SqliteFailure(ref err, _)) if err.extended_code == 2067 => {
                Err(MenteeError::UniqueViolation(name))
            }
            Err(err) => Err(MenteeError::from(err)),
        }
    }

    pub fn get_mentees_summaries(&self, show_all: bool) -> Result<Vec<MenteeSummary>, MenteeError> {
        match self.mentee_repo.get_all_mentees(show_all) {
            Ok(mentees) => Ok(mentees),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }

    pub fn get_mentee_with_counts(&self, name: String) -> Result<MenteeWithCounts, MenteeError> {
        match self.mentee_repo.get_mentee_with_counts(&name) {
            Ok(mentee) => Ok(mentee),
            Err(_) => Err(MenteeError::NotFound(format!("Mentee with name {}", name))),
        }
    }

    // TODO: handle cascade deletes
    pub fn delete_mentee(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = self.mentee_repo.get_mentee_id(&name)?.ok_or_else(|| {
            MenteeError::NotFound(format!("No mentee found with name '{}'", name))
        })?;

        match self.mentee_repo.delete_mentee_by_id(mentee_id) {
            Ok(_) => Ok(format!("Deleted mentee {}", name)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }

    pub fn get_mentee_count(&self, count: Option<CountOptions>) -> Result<String, MenteeError> {
        let message = match count {
            Some(CountOptions::Calls) => "Number of calls: ",
            Some(CountOptions::Gross) => "Gross $",
            Some(CountOptions::Net) => "Net $",
            Some(CountOptions::NetPerCall) => "Average net amount per call $",
            _ => "Number of mentees: ",
        };

        let count_value = self.mentee_repo.get_mentee_count(count)?;

        Ok(format!("{}{}", message, count_value))
    }

    pub fn update_mentee(&self, update_args: UpdateMentee) -> Result<String, MenteeError> {
        if update_args.new_name.is_none()
            && update_args.calls.is_none()
            && update_args.gross.is_none()
            && update_args.net.is_none()
            && update_args.status.is_none()
            && update_args.payment_day.is_none()
            && update_args.notes.is_none()
        {
            return Err(MenteeError::InvalidInput(
                "At least one field must be updated.".to_string(),
            ));
        }

        let rows_affected = self.mentee_repo.update_mentee(&update_args)?;

        if rows_affected == 0 {
            Err(MenteeError::NotFound(update_args.name))
        } else {
            Ok(format!(
                "{} was updated",
                update_args.new_name.as_deref().unwrap_or(&update_args.name)
            ))
        }
    }
}
