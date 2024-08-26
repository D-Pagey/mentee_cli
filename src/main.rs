use std::process;

fn main() {
    // check if db exists
    // if not prompt user for a path for db
    // provide option to cancel
    // pass db connection to run function? or pass db path?

    if let Err(e) = mentees::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

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

// println!("{:?}", args)

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
//
// use clitable::{format::Justify, print_stdout, Cell, Style, Table};
// use rusqlite::{Connection, Result};

// struct Mentee {
//     id: i32,
//     name: String,
//     calls_per_month: i32,
// }

// Name of the mentee
//     #[arg(short, long)]
//     name: Option<String>,
