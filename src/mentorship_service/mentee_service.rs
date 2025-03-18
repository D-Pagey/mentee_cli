use std::cell::RefCell;
use std::rc::Rc;
use std::usize;

use crate::models::mentee::Status;
use crate::utils::validation::{inquire_validate_day, inquire_validate_name};
use crate::UpdateMentee;
use crate::{constants, error::MenteeError};

use inquire::{CustomType, Select, Text};
use rusqlite::{Connection, Result};

pub struct MenteeService {
    conn: Rc<RefCell<Connection>>,
}

fn select_status() -> Result<Status, MenteeError> {
    // generate options from enum variants
    let options = Status::variants();
    let selected = Select::new("Select the mentee's status", options).prompt()?;

    Status::from_str(&selected).ok_or_else(|| "Invalid status selected".into())
}

impl MenteeService {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        Ok(Self { conn })
    }

    pub fn update_mentee_with_flags(
        &self,
        update_args: UpdateMentee,
    ) -> Result<String, MenteeError> {
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(new_name) = &update_args.new_name {
            updates.push("name = ?");
            params.push(Box::new(new_name));
        }

        if let Some(calls) = update_args.calls {
            updates.push("calls = ?");
            params.push(Box::new(calls));
        }

        if let Some(gross) = update_args.gross {
            updates.push("gross = ?");
            params.push(Box::new(gross));
        }

        if let Some(net) = update_args.net {
            updates.push("net = ?");
            params.push(Box::new(net));
        }

        if let Some(status) = update_args.status {
            updates.push("status = ?");
            params.push(Box::new(status.as_str()));
        }

        if let Some(payment_day) = update_args.payment_day {
            updates.push("payment_day = ?");
            params.push(Box::new(payment_day));
        }

        if let Some(notes) = update_args.notes {
            updates.push("notes = ?");
            params.push(Box::new(notes));
        }

        // Join updates and generate the SQL query
        let updates_str = updates.join(", ");

        let sql = format!(
            "UPDATE {} SET {} WHERE name = ?",
            constants::MENTEES_TABLE,
            updates_str
        );

        // Convert the params into the correct type
        let mut params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|s| s.as_ref()).collect();

        // Append the mentee's name as the last parameter.
        params_refs.push(&update_args.name);

        let rows_affected = self.conn.borrow().execute(&sql, params_refs.as_slice())?;

        if rows_affected == 0 {
            return Err(MenteeError::NotFound(update_args.name));
        } else {
            Ok(format!(
                "{} was updated",
                update_args.new_name.as_deref().unwrap_or(&update_args.name)
            ))
        }
    }

    fn generate_update_query(
        &self,
        name: &str,
        selected: &str,
    ) -> Result<(usize, Option<String>), MenteeError> {
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        // only so I can create a better success message
        let mut new_name: Option<String> = None;

        match selected {
            "Name" => {
                let name = Text::new("What is their name?")
                    .with_validator(inquire_validate_name)
                    .prompt()?;
                updates.push("name = ?");
                params.push(Box::new(name.clone()));
                new_name = Some(name);
            }
            "Calls" => {
                let calls = inquire::prompt_u32("How many calls per month do they have?")?;
                updates.push("calls = ?");
                params.push(Box::new(calls));
            }
            "Gross amount" => {
                let gross = inquire::prompt_u32("What is the gross payment?")?;
                updates.push("gross = ?");
                params.push(Box::new(gross));
            }
            "Net amount" => {
                let net = inquire::prompt_u32("What is the net payment?")?;
                updates.push("net = ?");
                params.push(Box::new(net));
            }
            "Status" => {
                let status = select_status()?;
                updates.push("status = ?");
                params.push(Box::new(status.as_str()));
            }
            "Payment Day" => {
                let payment_day: u32 = CustomType::new("Which day of the month do they pay?")
                    .with_validator(inquire_validate_day)
                    .prompt()?;
                updates.push("payment_day = ?");
                params.push(Box::new(payment_day));
            }
            "Notes" => {
                let notes = Text::new("Any notes?").prompt()?;
                updates.push("notes = ?");
                params.push(Box::new(notes));
            }
            _ => {
                return Err(MenteeError::InvalidInput(
                    "Invalid select option".to_string(),
                ))
            }
        }

        // Join the updates
        let updates_str = updates.join(", ");

        // Construct the SQL query
        let sql = format!(
            "UPDATE {} SET {} WHERE name = ?",
            constants::MENTEES_TABLE,
            updates_str
        );

        // Add the mentee's current name to the params (for the WHERE clause)
        params.push(Box::new(name));

        // Convert params into the correct type
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        // Execute the SQL query
        let updated = self.conn.borrow().execute(&sql, params_refs.as_slice())?;

        Ok((updated, new_name))
    }

    pub fn update_mentee_interactive(&self, name: String) -> Result<String, MenteeError> {
        let options = vec![
            "Name",
            "Calls",
            "Gross amount",
            "Net amount",
            "Status",
            "Payment Day",
            "Notes",
        ];

        let selected = Select::new("Which property do you want to update?", options).prompt()?;

        let (rows_affected, new_name) = self.generate_update_query(&name, selected)?;

        if rows_affected == 0 {
            return Err(MenteeError::NotFound(name));
        } else {
            Ok(format!(
                "{} was updated",
                new_name.as_deref().unwrap_or(&name)
            ))
        }
    }
}
