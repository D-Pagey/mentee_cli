use inquire::Text;
use rusqlite::Connection;

pub fn update_mentee(conn: &Connection, name: String) {
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
