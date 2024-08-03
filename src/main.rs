use clap::Parser;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "CLI tool to manage mentees", long_about = None, name = "Mentee CLI")]
struct Args {
    /// Name of the mentee
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    let table = vec![
        vec!["AA".cell(), "1 year".cell().justify(Justify::Right)],
        vec!["MD".cell(), "6 months".cell().justify(Justify::Right)],
        vec!["MS".cell(), "3 months".cell().justify(Justify::Right)],
        vec!["PM".cell(), "5 months".cell().justify(Justify::Right)],
        vec!["AL".cell(), "8 months".cell().justify(Justify::Right)],
        vec!["DG".cell(), "4 months".cell().justify(Justify::Right)],
    ]
    .table()
    .title(vec![
        "Initials".cell().bold(true),
        "Duration".cell().bold(true),
    ])
    .bold(true);

    // assert!(print_stdout(table).is_ok());
    // TODO: if name arg then fetch that mentee
    // else just render entire table
    match args.name {
        Some(name) => println!("Hello {}", name),
        None => print_stdout(table).expect("Error reading table"),
    }
}
