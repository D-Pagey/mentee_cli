use clap::{Parser, Subcommand};
use std::error::Error;

/// CLI to manage state of mentees
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "Mentee CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Creates a new mentee
    Create,
    /// Deletes an existing mentee
    Delete,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create => println!("Creating a new mentee"),
        Commands::Delete => println!("Deleting a mentee..."),
    }
    Ok(())
}
