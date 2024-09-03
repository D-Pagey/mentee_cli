use cli_table::{format::Justify, Cell, Style, Table};
use rusqlite::{Connection, Result};

use crate::Mentee;

pub fn get_mentees(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT name, calls FROM mentee")?;
    let mentee_iter = stmt.query_map([], |row| {
        Ok(Mentee {
            name: row.get(0)?,
            calls: row.get(1)?,
        })
    })?;

    let mut mentees: Vec<Mentee> = Vec::new();

    for mentee_result in mentee_iter {
        mentees.push(mentee_result?)
    }

    let table = mentees
        .into_iter()
        .map(|mentee| {
            vec![
                mentee.name.cell(),
                mentee.calls.cell().justify(Justify::Right),
            ]
        })
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Calls / Month".cell().bold(true),
        ])
        .bold(true);

    // TODO: change unwrap to handle error
    let table_display = table.display().unwrap();

    println!("{}", table_display);

    Ok(())
}
