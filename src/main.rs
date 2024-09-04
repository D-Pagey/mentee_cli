use rusqlite::Result;
use std::process;

fn main() -> Result<()> {
    if let Err(e) = mentees::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
