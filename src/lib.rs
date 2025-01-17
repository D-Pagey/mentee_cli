pub mod call_service;
mod cli;
mod constants;
mod error;
mod mentee;
pub mod mentee_service;
mod utils;

use call_service::CallService;
use clap::{Parser, Subcommand, ValueEnum};
use cli::{render_calls_table, render_mentees_table};
use error::MenteeError;
use mentee::Status;
use mentee_service::MenteeService;
use rusqlite::Result;
use utils::{clap_validate_day, clap_validate_name};

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
    List {
        /// Include archived mentees which are hidden by default
        #[arg(long, default_value_t = false)]
        all: bool,
    },
    /// Adds a new mentee
    Add,
    /// Updates an existing mentee
    Update(UpdateMentee),
    /// Deletes a mentee
    Delete { name: String },
    /// Count or Sum a specified column
    Count { column: Option<CountOptions> },
    /// Manage calls
    Calls {
        #[command(subcommand)]
        action: CallActions,
    },
}

#[derive(Subcommand, Debug, Clone)]
enum CallActions {
    /// List all calls
    List,
    /// Add a call
    Add,
}

#[derive(Parser, Clone, Debug)]
pub struct UpdateMentee {
    /// The current name of the mentee (Required)
    pub name: String,

    /// Optionally update the name
    #[arg(long, value_parser = clap_validate_name)]
    pub new_name: Option<String>,

    /// Optionally update the number of calls
    #[arg(long)]
    pub calls: Option<i32>,

    /// Optionally update the status
    #[arg(long)]
    pub status: Option<Status>,

    /// Optionally update the day the mentee pays
    #[arg(long, value_parser = clap_validate_day)]
    pub payment_day: Option<i32>,

    /// Optionally update the gross amount
    #[arg(long)]
    pub gross: Option<i32>,

    /// Optionally update the net amount
    #[arg(long)]
    pub net: Option<i32>,

    /// Optionally update the notes
    #[arg(long)]
    pub notes: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CountOptions {
    Mentees,
    Calls,
    Gross,
    Net,
    NetPerCall,
}

fn as_debug<T: std::fmt::Debug>(option: &Option<T>) -> Option<&dyn std::fmt::Debug> {
    option.as_ref().map(|value| value as &dyn std::fmt::Debug)
}

pub fn run() -> Result<(), MenteeError> {
    let mentee_service = MenteeService::new(false)?;
    let call_service = CallService::new()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::List { all } => {
            if let Err(err) = mentee_service
                .get_all_mentees(all)
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
                as_debug(&update_args.notes),
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
        Commands::Calls { action } => match action {
            CallActions::List => {
                if let Err(err) = call_service.get_all_calls().and_then(render_calls_table) {
                    eprintln!("{err}");
                }
            }
            CallActions::Add => match call_service.add_call() {
                Ok(call) => println!("{:?}", call),
                Err(err) => eprintln!("{err}"),
            },
        },
    };

    Ok(())
}
