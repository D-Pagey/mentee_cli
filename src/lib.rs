mod constants;
mod mentee;
mod mentee_service;

use clap::{Parser, Subcommand};
use rusqlite::Result;
use std::error::Error;

use mentee_service::MenteeService;

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

// TODO: is there a better / or more accurate error type
pub fn run() -> Result<(), Box<dyn Error>> {
    let database_url = "mentees.db";
    let mentee_service = MenteeService::new(database_url)?;

    let cli = Cli::parse();

    // TODO: should these handle the errors themselves or deal with them?
    // TODO: service should return what needs rendering
    match cli.command {
        Commands::List => mentee_service.get_all_mentees()?,
        Commands::Add => {
            let mentee = mentee_service.add_mentee()?;
            println!("Added Mentee: {}", mentee.name);
        }
        Commands::Update { name } => mentee_service.update_mentee(name), // TODO: what should this return?
        Commands::Delete { name } => mentee_service.delete_mentee(name),
    }

    Ok(())
}
