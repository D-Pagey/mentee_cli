mod cli;
mod constants;
mod error;
mod mentee;
mod mentee_service;

use clap::{Parser, Subcommand};
use cli::render_mentees_table;
use error::MenteeError;
use mentee_service::MenteeService;
use rusqlite::Result;

/// CLI to manage state of mentees
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "Mentee CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// List all mentees
    List,
    /// Adds a new mentee
    Add,
    /// Updates an existing mentee
    Update { name: String },
    /// Deletes a mentee
    Delete { name: String },
}

pub fn run() -> Result<(), MenteeError> {
    let database_url = "mentees.db";
    let mentee_service = MenteeService::new(database_url)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let result = mentee_service.get_all_mentees();

            match result {
                Ok(mentees) => {
                    let render_result = render_mentees_table(mentees);

                    match render_result {
                        Ok(()) => (),
                        Err(err) => eprintln!("{err}"),
                    };
                }
                Err(err) => eprintln!("{err}"),
            }
        }
        Commands::Add => {
            let result = mentee_service.add_mentee();

            match result {
                Ok(mentee) => println!("Added Mentee: {}", mentee.name),
                Err(err) => eprintln!("{err}"),
            }
        }
        Commands::Update { name } => {
            let result = mentee_service.update_mentee(name); // TODO: what should this return?
            match result {
                Ok(updated) => println!("Updated Mentee: {}", updated),
                Err(err) => eprintln!("{err}"),
            }
        }
        Commands::Delete { name } => {
            let result = mentee_service.delete_mentee(name);

            match result {
                Ok(deleted) => println!("Deleted Mentee: {}", deleted),
                Err(err) => eprintln!("{err}"),
            }
        }
    };

    Ok(())
}
