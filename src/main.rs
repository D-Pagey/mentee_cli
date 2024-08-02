use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about = "CLI tool to manage mentees", long_about = None, name = "Mentee CLI")]
struct Args {
    /// Name of the mentee
    #[arg(short, long)]
    name: Option<String>,
    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

fn main() {
    let args = Args::parse();

    // for _ in 0..args.count {
    // println!("Hello {:?}!", args.name);
    // }

    match args.name {
        Some(name) => println!("Hello {}", name),
        None => println!("No name provided"),
    }
    //
    //
    //
    // if (args.name) {
    //     println!("Hello {:?}!", args.name);
    // } else {
    //     println!("No name given");
    // }
    // TODO: if name arg then fetch that mentee
    // else just render entire table
}
