use rusqlite::Connection;

pub fn delete_mentee(conn: &Connection, name: String) {
    match conn.execute(
        "DELETE FROM mentee WHERE name = :name",
        &[(":name", &name.to_lowercase())],
    ) {
        Ok(deleted) => {
            if deleted == 0 {
                println!("There are no mentees by the name of {name}");
            } else {
                println!("Deleted all mentees called {name} ({deleted})");
            }
        }
        Err(error) => eprintln!("Error deleting {name} - {error}"),
    }
}
