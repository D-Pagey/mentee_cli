use crate::utils::{inquire_validate_day, inquire_validate_name};
use crate::{constants, error::MenteeError};
use crate::{CountOptions, UpdateMentee};

use crate::mentee::{Mentee, Status};
use inquire::{CustomType, Select, Text};
use rusqlite::{Connection, Result};

pub struct MenteeService {
    conn: Connection,
}

fn select_status() -> Result<Status, MenteeError> {
    // generate options from enum variants
    let options = Status::variants();
    let selected = Select::new("Select the mentee's status", options).prompt()?;

    Status::from_str(&selected).ok_or_else(|| "Invalid status selected".into())
}

fn calc_net_per_call(net: u32, calls: u32) -> u32 {
    if calls == 0 {
        net
    } else {
        net / calls
    }
}

impl MenteeService {
    pub fn new(database_url: &str) -> Result<Self, MenteeError> {
        let conn = Connection::open(database_url)?;

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            calls INTEGER,
            gross INTEGER NOT NULL,
            net INTEGER NOT NULL,
            net_per_call INTEGER NOT NULL,
            status TEXT NOT NULL CHECK(status IN ('archived', 'cold', 'warm', 'hot')),
            payment_day INTEGER NOT NULL CHECK(payment_day BETWEEN 1 AND 31))",
            constants::MENTEE_TABLE
        );

        conn.execute(&sql, ())?;

        Ok(MenteeService { conn })
    }

    pub fn add_mentee(&self) -> Result<Mentee, MenteeError> {
        let name = Text::new("What is their name?")
            .with_validator(inquire_validate_name)
            .prompt()?;
        let calls = inquire::prompt_u32("How many calls per month do they have?")?;
        let gross = inquire::prompt_u32("What is the gross payment?")?;
        let net = inquire::prompt_u32("What is the net payment?")?;
        let net_per_call = calc_net_per_call(net, calls);
        let status = select_status()?;
        let payment_day: u32 = CustomType::new("Which day of the month do they pay?")
            .with_validator(inquire_validate_day)
            .prompt()?;

        let mentee = Mentee {
            name,
            calls,
            gross,
            net,
            net_per_call,
            status,
            payment_day,
        };

        let result = self.conn.execute(
            &format!(
                "INSERT INTO {} (name, calls, gross, net, net_per_call, status, payment_day) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                constants::MENTEE_TABLE
            ),
            (
                &mentee.name,
                &mentee.calls,
                &mentee.gross,
                &mentee.net,
                &mentee.net_per_call,
                Status::as_str(&mentee.status),
                &mentee.payment_day,
            ),
        );

        match result {
            Ok(_) => Ok(mentee),
            Err(rusqlite::Error::SqliteFailure(ref err, _)) if err.extended_code == 2067 => {
                Err(MenteeError::UniqueViolation(mentee.name))
            }
            Err(err) => Err(MenteeError::from(err)),
        }
    }

    pub fn delete_mentee(&self, name: String) -> Result<usize, MenteeError> {
        let deleted = self.conn.execute(
            &format!("DELETE FROM {} WHERE name = :name", constants::MENTEE_TABLE),
            &[(":name", &name.to_lowercase())],
        )?;

        Ok(deleted)
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

        // Join updates and generate the SQL query
        let updates_str = updates.join(", ");

        let sql = format!(
            "UPDATE {} SET {} WHERE name = ?",
            constants::MENTEE_TABLE,
            updates_str
        );

        // Convert the params into the correct type
        let mut params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|s| s.as_ref()).collect();

        // Append the mentee's name as the last parameter.
        params_refs.push(&update_args.name);

        let rows_affected = self.conn.execute(&sql, params_refs.as_slice())?;

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
            constants::MENTEE_TABLE,
            updates_str
        );

        // Add the mentee's current name to the params (for the WHERE clause)
        params.push(Box::new(name));

        // Convert params into the correct type
        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        // Execute the SQL query
        let updated = self.conn.execute(&sql, params_refs.as_slice())?;

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

    pub fn get_all_mentees(&self) -> Result<Vec<Mentee>, MenteeError> {
        let sql = format!("SELECT * FROM {}", constants::MENTEE_TABLE);
        let mut stmt = self.conn.prepare(&sql)?;
        let mentee_iter = stmt.query_map([], |row| {
            let status_str: String = row.get(6)?;

            let status = Status::from_str(&status_str).unwrap_or(Status::Warm);

            Ok(Mentee {
                name: row.get(1)?,
                calls: row.get(2)?,
                gross: row.get(3)?,
                net: row.get(4)?,
                net_per_call: row.get(5)?,
                status,
                payment_day: row.get(7)?,
            })
        })?;

        let mut mentees: Vec<Mentee> = Vec::new();

        for mentee_result in mentee_iter {
            mentees.push(mentee_result?)
        }

        Ok(mentees)
    }

    pub fn get_mentee_count(&self, count: Option<CountOptions>) -> Result<String, MenteeError> {
        let (sql, message) = match count {
            Some(CountOptions::Calls) => ("SELECT SUM(calls) FROM mentees", "Number of calls: "),
            Some(CountOptions::Gross) => ("SELECT SUM(gross) FROM mentees", "Gross $"),
            Some(CountOptions::Net) => ("SELECT SUM(net) FROM mentees", "Net $"),
            _ => ("SELECT COUNT(*) FROM mentees", "Number of mentees: "),
        };

        let result: i64 = self.conn.query_row(sql, [], |row| row.get(0))?;

        Ok(format!("{}{}", message, result))
    }
}
