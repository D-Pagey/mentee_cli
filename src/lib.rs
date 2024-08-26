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
        Commands::Show => get_all_mentees(&conn)?,
        Commands::Create => create_mentee(&conn)?,
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
    let mut stmt = conn.prepare("SELECT id, name, calls_per_month FROM mentee")?;
    let mentee_iter = stmt.query_map([], |row| {
        Ok(Mentee {
            id: row.get(0)?,
            name: row.get(1)?,
            calls_per_month: row.get(2)?,
        })
    })?;

    let _table: Vec<Mentee> = Vec::new();

    for mentee in mentee_iter {
        println!("Found mentee {:?}", mentee.unwrap());
        // table.push(mentee)
    }

    // let table = vec![
    //     vec!["AA".cell(), "1 year".cell().justify(Justify::Right)],
    //     vec!["MD".cell(), "6 months".cell().justify(Justify::Right)],
    //     vec!["MS".cell(), "3 months".cell().justify(Justify::Right)],
    //     vec!["PM".cell(), "5 months".cell().justify(Justify::Right)],
    //     vec!["AL".cell(), "8 months".cell().justify(Justify::Right)],
    //     vec!["DG".cell(), "4 months".cell().justify(Justify::Right)],
    // ]
    // .table()
    // .title(vec![
    //     "Initials".cell().bold(true),
    //     "Duration".cell().bold(true),
    // ])
    // .bold(true);
    // assert!(print_stdout(table).is_ok());
    Ok(())
}

fn create_mentee(conn: &Connection) -> Result<()> {
    let alex = Mentee {
        id: 0,
        name: "alex".to_string(),
        calls_per_month: 2,
    };

    conn.execute(
        "INSERT INTO mentee (name, calls_per_month) VALUES (?1, ?2)",
        (&alex.name, &alex.calls_per_month),
    )?;

    println!("mentee created");

    Ok(())
}
