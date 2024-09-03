use clap::{Parser, Subcommand};
use cli_table::{format::Justify, Cell, Style, Table};
use inquire::Text;
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
    /// Adds a new mentee
    Add,
    /// Updates an existing mentee
    Update { name: String },
    /// Deletes a mentee
    Delete { name: String },
}

// TODO: is there a better / or more accurate error type
pub fn run(conn: Connection) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // TODO: should these handle the errors themselves or deal with them?
    match cli.command {
        Commands::Show => get_all_mentees(&conn)?,
        Commands::Add => add_mentee(&conn)?,
        Commands::Update { name } => update_mentee(&conn, name),
        Commands::Delete { name } => delete_mentee(&conn, name),
    }

    Ok(())
}

#[derive(Debug)]
struct Mentee {
    name: String,
    calls: u32,
}

fn get_all_mentees(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, calls FROM mentee")?;
    let mentee_iter = stmt.query_map([], |row| {
        Ok(Mentee {
            name: row.get(0)?,
            calls: row.get(1)?,
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
                mentee.calls.cell().justify(Justify::Right),
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

// TODO: remove unwraps, deal with ? for conn.execute, add error to result type
fn add_mentee(conn: &Connection) -> Result<()> {
    // TODO: why Text::new over inquire.prompt
    let name = Text::new("What is their name?")
        .prompt()
        // TODO: unwrap, unwrap or else, expect, ?, whats the best?
        // should this pass the error upto handle since they all
        // are as bad as each other?
        .expect("Failed to capture mentee name");
    let calls = inquire::prompt_u32("How many calls per month do they have?")
        .expect("Failed to capture mentee calls");

    let mentee = Mentee { name, calls };

    conn.execute(
        "INSERT INTO mentee (name, calls) VALUES (?1, ?2)",
        (&mentee.name, &mentee.calls),
    )?;

    println!("{} added", mentee.name);

    // match name {
    //     Ok(name) => println!("Your name is being published...{}", name),
    //     Err(err) => println!("Error while publishing...{}", err),
    // }

    // TODO: add validator to parse to number then check max calls

    // how to use the parsing_u32
    // let calls = Text::new("How many calls per month do they have?").prom

    // match calls {
    //     Ok(calls) => println!("Your calls is being published...{}", calls),
    //     Err(err) => println!("Error while publishing...{}", err),
    // }
    // ::build vs ::new
    // the struct implementation validates the number of calls
    // returns valid error message i.e too many calls
    //
    //

    Ok(())
}

fn delete_mentee(conn: &Connection, name: String) {
    match conn.execute(
        "DELETE FROM mentee WHERE name = :name",
        &[(":name", &name.to_lowercase())],
    ) {
        Ok(deleted) => {
            if deleted == 0 {
                println!("There are no mentees by the name of {name}");
            } else {
                println!("Deleted all mentees called {name} ({deleted})");
            }
        }
        Err(error) => eprintln!("Error deleting {name} - {error}"),
    }
}

fn update_mentee(conn: &Connection, name: String) {
    let calls = Text::new("How many calls per month do they have?")
        .prompt()
        .expect("Failed to capture mentee name");

    match conn.execute(
        "UPDATE mentee SET calls = ?1 WHERE name = ?2",
        (&calls, &name),
    ) {
        Ok(updated) => println!("updated...{updated}"),
        Err(error) => println!("you fucked up - {error}"),
    }
}
