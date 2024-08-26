use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
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
    /// Show all mentees
    Show,
    /// Creates a new mentee
    Create,
    /// Deletes an existing mentee
    Delete,
}

// TODO: is there a better error type
pub fn run(conn: Connection) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show => get_all_mentees(&conn).unwrap(),
        Commands::Create => println!("Creating a new mentee"),
        Commands::Delete => println!("Deleting a mentee..."),
    }

    Ok(())
}

#[derive(Debug)]
struct Mentee {
    id: i32,
    name: String,
    calls_per_month: i32,
}

fn get_all_mentees(conn: &Connection) -> Result<()> {
    let alex = Mentee {
        id: 0,
        name: "alex".to_string(),
        calls_per_month: 2,
    };

    conn.execute(
        "INSERT INTO mentee (name, calls_per_month) VALUES (?1, ?2)",
        (&alex.name, &alex.calls_per_month),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, calls_per_month FROM mentee")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Mentee {
            id: row.get(0)?,
            name: row.get(1)?,
            calls_per_month: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
