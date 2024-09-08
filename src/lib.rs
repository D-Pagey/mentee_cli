mod cli;
mod constants;
mod error;
mod mentee;
mod mentee_service;

use clap::{Parser, Subcommand, ValueEnum};
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
    Update(UpdateMentee),
    /// Deletes a mentee
    Delete { name: String },
    /// Count or Sum a specified column
    Count { column: Option<ColumnOptions> },
}

#[derive(Parser, Clone, Debug)]
pub struct UpdateMentee {
    /// The current name of the mentee (Required)
    pub name: String,

    /// Optionally update the name
    #[arg(long)]
    pub new_name: Option<String>,

    /// Optionally update the number of calls
    #[arg(long)]
    pub calls: Option<String>,

    /// Optionally update the status
    #[arg(long)]
    pub status: Option<String>,

    /// Optionally update the day the mentee pays
    #[arg(long)]
    pub payment_day: Option<String>,

    /// Optionally update the gross amount
    #[arg(long)]
    pub gross: Option<String>,

    /// Optionally update the net amount
    #[arg(long)]
    pub net: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ColumnOptions {
    Mentees,
    Calls,
    Gross,
    Net,
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
            let has_any_flags = vec![
                update_args.new_name.as_ref(),
                update_args.calls.as_ref(),
                update_args.gross.as_ref(),
                update_args.net.as_ref(),
                update_args.status.as_ref(),
                update_args.payment_day.as_ref(),
            ]
            .into_iter()
            .map(|opt| opt.is_some())
            .any(|x| x);

            if has_any_flags {
                match mentee_service.update_mentee_with_flags(update_args) {
                    Ok(confirmation) => println!("{}", confirmation),
                    Err(err) => eprintln!("{err}"),
                };
            } else {
                match mentee_service.update_mentee_interactive(update_args.name) {
                    Ok(name) => println!("Updated {}", name),
                    Err(err) => eprintln!("{err}"),
                };
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
