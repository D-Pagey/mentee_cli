use clap::{Parser, Subcommand};
use cli_table::{format::Justify, Cell, Style, Table};
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
        Commands::Show => get_all_mentees(&conn)?,
        Commands::Create => create_mentee(&conn)?,
        Commands::Delete => println!("Deleting a mentee..."),
    }

    Ok(())
}

#[derive(Debug)]
struct Mentee {
    name: String,
    calls_per_month: i32,
}

fn get_all_mentees(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, calls_per_month FROM mentee")?;
    let mentee_iter = stmt.query_map([], |row| {
        Ok(Mentee {
            name: row.get(0)?,
            calls_per_month: row.get(1)?,
        })
    })?;

    let mut mentees: Vec<Mentee> = Vec::new();

    for mentee_result in mentee_iter {
        mentees.push(mentee_result?)
    }

    let table = mentees
        .into_iter()
        .map(|mentee| {
            vec![
                mentee.name.cell(),
                mentee.calls_per_month.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Calls / Month".cell().bold(true),
        ])
        .bold(true);

    // TODO: change unwrap to handle error
    let table_display = table.display().unwrap();

    println!("{}", table_display);

    Ok(())
}

fn create_mentee(conn: &Connection) -> Result<()> {
    // ::build vs ::new
    // the struct implementation validates the number of calls
    // returns valid error message i.e too many calls
    let mentee = Mentee {
        name: "alex".to_string(),
        calls_per_month: 2,
    };

    conn.execute(
        "INSERT INTO mentee (name, calls_per_month) VALUES (?1, ?2)",
        (&mentee.name, &mentee.calls_per_month),
    )?;

    println!("mentee created");

    Ok(())
}
