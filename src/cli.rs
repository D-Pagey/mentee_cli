use cli_table::{format::Justify, Cell, Style, Table};

use crate::{error::MenteeError, mentee::Mentee};

pub fn render_mentees_table(mentees: Vec<Mentee>) -> Result<(), MenteeError> {
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

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}
