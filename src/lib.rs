mod cli;
mod config;
mod constants;
mod db;
mod error;
mod models;
mod repositories;
mod services;
mod utils;

use clap::{Parser, Subcommand, ValueEnum};
use cli::{
    display_mentee, render_calls_table, render_mentees_table, render_payments_table,
    render_videos_table,
};
use config::Config;
use db::connection;
use db::migrations;
use error::MenteeError;
use models::mentee::Status;
use rusqlite::Result;
use services::CallService;
use services::MenteeService;
use services::PaymentService;
use services::VideoService;
use utils::validation::clap_validate_day;
use utils::validation::clap_validate_name;

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
    /// View more details of a mentee
    View { name: String },
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
    /// Manage video analysis
    Videos {
        #[command(subcommand)]
        action: VideoActions,
    },
    /// Manage payments
    Payments {
        #[command(subcommand)]
        action: PaymentActions,
    },
}

// TODO: deduplicate
#[derive(Subcommand, Debug, Clone)]
enum CallActions {
    /// List all calls
    List { name: Option<String> },
    /// Add a call
    Add { name: String },
    /// Delete a call
    Delete { call_id: u32 },
    /// Update a call log
    Update { call_id: u32 },
}

#[derive(Subcommand, Debug, Clone)]
enum PaymentActions {
    /// List all payments
    List { name: Option<String> },
    /// Add a payment
    Add { name: String },
    /// Delete a payment
    Delete { payment_id: u32 },
    /// Update a payment
    Update { payment_id: u32 },
}

#[derive(Subcommand, Debug, Clone)]
enum VideoActions {
    /// List all analysis videos
    List { name: Option<String> },
    /// Add a video analysis
    Add { name: String },
    /// Delete a video log
    Delete { video_id: u32 },
    /// Update a video log
    Update { video_id: u32 },
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

pub fn run() -> Result<(), MenteeError> {
    let config = Config::new()?;
    let conn =
        connection::establish_connection(&config).expect("Failed to connect to the database");

    migrations::run_migrations(&conn).expect("Failed to run database migrations");

    let call_service = CallService::new(&conn);
    let mentee_service = MenteeService::new(&conn);
    let payment_service = PaymentService::new(&conn);
    let video_service = VideoService::new(&conn);

    let cli = Cli::parse();

    match cli.command {
        Commands::List { all } => {
            if let Err(err) = mentee_service
                .get_mentees_summaries(all)
                .and_then(render_mentees_table)
            {
                eprintln!("{err}");
            }
        }
        Commands::View { name } => match mentee_service.get_mentee_with_counts(name) {
            Ok(mentee) => display_mentee(mentee),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Add => match mentee_service.add_mentee() {
            Ok(name) => println!("Added Mentee: {}", name),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Update(update_args) => match mentee_service.update_mentee(update_args) {
            Ok(message) => println!("{}", message),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Delete { name } => match mentee_service.delete_mentee(name) {
            Ok(deleted) => println!("Deleted Mentee: {}", deleted),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Count { column } => match mentee_service.get_mentee_count(column) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Calls { action } => match action {
            CallActions::List { name } => match call_service.get_all_calls(name) {
                Ok(calls) => render_calls_table(calls)?,
                Err(err) => eprintln!("{}", err),
            },
            CallActions::Add { name } => match call_service.add_call(name) {
                Ok(success) => println!("{success}"),
                Err(err) => eprintln!("{err}"),
            },
            CallActions::Update { call_id } => match call_service.update_call(call_id) {
                Ok(success) => println!("{success}"),
                Err(err) => eprintln!("{err}"),
            },
            CallActions::Delete { call_id } => {
                match call_service.delete_call(call_id) {
                    Ok(deleted) => println!("{deleted}"),
                    Err(err) => eprintln!("{err}"),
                };
            }
        },
        Commands::Videos { action } => match action {
            VideoActions::List { name } => match video_service.get_all_videos(name) {
                Ok(videos) => render_videos_table(videos)?,
                Err(err) => eprintln!("{}", err),
            },
            VideoActions::Add { name } => match video_service.add_video(name) {
                Ok(success) => println!("{success}"),
                Err(err) => eprintln!("{err}"),
            },
            VideoActions::Update { video_id } => match video_service.update_video(video_id) {
                Ok(success) => println!("{success}"),
                Err(err) => eprintln!("{err}"),
            },
            VideoActions::Delete { video_id } => match video_service.delete_video(video_id) {
                Ok(deleted) => println!("{deleted}"),
                Err(err) => eprintln!("{err}"),
            },
        },
        Commands::Payments { action } => match action {
            PaymentActions::List { name } => match payment_service.get_all_payments(name) {
                Ok(payments) => render_payments_table(payments)?,
                Err(err) => eprintln!("{}", err),
            },
            PaymentActions::Add { name } => match payment_service.add_payment(name) {
                Ok(success) => println!("{success}"),
                Err(err) => eprintln!("{err}"),
            },
            PaymentActions::Update { payment_id } => {
                match payment_service.update_payment(payment_id) {
                    Ok(success) => println!("{success}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
            PaymentActions::Delete { payment_id } => {
                match payment_service.delete_payment(payment_id) {
                    Ok(success) => println!("{success}"),
                    Err(err) => eprintln!("{err}"),
                }
            }
        },
    };

    Ok(())
}
