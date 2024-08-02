use clap::Parser;

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

    // TODO: if name arg then fetch that mentee
    // else just render entire table
    match args.name {
        Some(name) => println!("Hello {}", name),
        None => println!("No name provided"),
    }
}
