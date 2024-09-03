use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::error::Error;

mod add_mentee;
mod delete_mentee;
mod get_mentees;
mod update_mentee;

/// CLI to manage state of mentees
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "Mentee CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Show all mentees
    Show,
    /// Adds a new mentee
    Add,
    /// Updates an existing mentee
    Update { name: String },
    /// Deletes a mentee
    Delete { name: String },
}

#[derive(Debug)]
pub struct Mentee {
    name: String,
    calls: u32,
}

// TODO: is there a better / or more accurate error type
pub fn run(conn: Connection) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // TODO: should these handle the errors themselves or deal with them?
    match cli.command {
        Commands::Show => get_mentees::get_mentees(&conn)?,
        Commands::Add => add_mentee::add_mentee(&conn)?,
        Commands::Update { name } => update_mentee::update_mentee(&conn, name),
        Commands::Delete { name } => delete_mentee::delete_mentee(&conn, name),
    }

    Ok(())
}
