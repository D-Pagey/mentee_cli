use clap::{Parser, Subcommand};
// use clitable::{format::Justify, print_stdout, Cell, Style, Table};
// use rusqlite::{Connection, Result};

/// CLI to manage state of mentees
#[derive(Parser, Debug)]
#[command(version, about = "CLI tool to manage mentees", long_about = None, name = "Mentee CLI")]
struct Args {
    // Name of the mentee
    //     #[arg(short, long)]
    //     name: Option<String>,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create,
    Delete,
}

// struct Mentee {
//     id: i32,
//     name: String,
//     calls_per_month: i32,
// }

// TODO: decide on error type
fn main() {
    // TODO: move this to a build type function or a ::new?
    // let conn = Connection::open("mentees.db")?;
    //
    // conn.execute(
    //     "CREATE TABLE IF NOT EXISTS mentee (
    //         id INTEGER PRIMARY KEY,
    //         name TEXT NOT NULL,
    //         calls_per_month INTEGER
    //     )",
    //     (),
    // )?;

    let args = Args::parse();

    println!("{:?}", args)

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
    // TODO: if name arg then fetch that mentee
    // else just render entire table
    // match args.name {
    //     Some(name) => println!("Hello {}", name),
    //     None => print_stdout(table).expect("Error reading table"),
    // }

    // Ok(())
}
