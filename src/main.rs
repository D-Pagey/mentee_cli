use std::process;

#[cfg(not(tarpaulin_include))]
fn main() {
    if let Err(e) = mentees::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
