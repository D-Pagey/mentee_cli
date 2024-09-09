mod cli;
mod constants;
mod error;
mod mentee;
mod mentee_service;

use clap::{Parser, Subcommand, ValueEnum};
use cli::render_mentees_table;
use error::MenteeError;
use mentee::Status;
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
    Update(UpdateMentee),
    /// Deletes a mentee
    Delete { name: String },
    /// Count or Sum a specified column
    Count { column: Option<CountOptions> },
}

#[derive(Parser, Clone, Debug)]
pub struct UpdateMentee {
    /// The current name of the mentee (Required)
    pub name: String,

    /// Optionally update the name
    #[arg(long, value_parser = validate_name)]
    pub new_name: Option<String>,

    /// Optionally update the number of calls
    #[arg(long)]
    pub calls: Option<i32>,

    /// Optionally update the status
    #[arg(long)]
    pub status: Option<Status>,

    /// Optionally update the day the mentee pays
    #[arg(long)]
    pub payment_day: Option<i32>,

    /// Optionally update the gross amount
    #[arg(long)]
    pub gross: Option<i32>,

    /// Optionally update the net amount
    #[arg(long)]
    pub net: Option<i32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CountOptions {
    Mentees,
    Calls,
    Gross,
    Net,
}

fn as_debug<T: std::fmt::Debug>(option: &Option<T>) -> Option<&dyn std::fmt::Debug> {
    option.as_ref().map(|value| value as &dyn std::fmt::Debug)
}

fn validate_name(s: &str) -> Result<String, String> {
    if s.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        Ok(s.to_string())
    } else {
        Err("Name can only contain letters and spaces.".to_string())
    }
}

pub fn run() -> Result<(), MenteeError> {
    let database_url = "mentees.db";
    let mentee_service = MenteeService::new(database_url)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            if let Err(err) = mentee_service
                .get_all_mentees()
                .and_then(render_mentees_table)
            {
                eprintln!("{err}");
            }
        }
        Commands::Add => match mentee_service.add_mentee() {
            Ok(mentee) => println!("Added Mentee: {}", mentee.name),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Update(update_args) => {
            let has_any_flags = [
                as_debug(&update_args.new_name),
                as_debug(&update_args.calls),
                as_debug(&update_args.gross),
                as_debug(&update_args.net),
                as_debug(&update_args.status),
                as_debug(&update_args.payment_day),
            ]
            .iter()
            .any(Option::is_some);

            let result = if has_any_flags {
                mentee_service.update_mentee_with_flags(update_args)
            } else {
                mentee_service.update_mentee_interactive(update_args.name)
            };

            match result {
                Ok(message) => println!("{}", message),
                Err(err) => eprintln!("{err}"),
            }
        }
        Commands::Delete { name } => match mentee_service.delete_mentee(name) {
            Ok(deleted) => println!("Deleted Mentee: {}", deleted),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Count { column } => match mentee_service.get_mentee_count(column) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("{err}"),
        },
    };

    Ok(())
}
