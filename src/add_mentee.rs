use inquire::Text;
use rusqlite::{Connection, Result};

use crate::Mentee;

// TODO: remove unwraps, deal with ? for conn.execute, add error to result type
pub fn add_mentee(conn: &Connection) -> Result<()> {
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
