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
            if let Err(err) = mentee_service
                .get_all_mentees_and_count()
                .and_then(render_mentees_table)
            {
                eprintln!("{err}");
            }
        }
        Commands::Add => match mentee_service.add_mentee() {
            Ok(mentee) => println!("Added Mentee: {}", mentee.name),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Update { name } => match mentee_service.update_mentee(name) {
            Ok(updated) => println!("Updated Mentee: {}", updated),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Delete { name } => match mentee_service.delete_mentee(name) {
            Ok(deleted) => println!("Deleted Mentee: {}", deleted),
            Err(err) => eprintln!("{err}"),
        },
    };

    Ok(())
}
