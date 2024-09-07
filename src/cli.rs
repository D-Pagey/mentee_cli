use cli_table::{format::Justify, Cell, Color, Style, Table};

use crate::{error::MenteeError, mentee::Mentee};

// TODO: dont love these args, cleaner way?
pub fn render_mentees_table((mentees, count): (Vec<Mentee>, i64)) -> Result<(), MenteeError> {
    let mut rows: Vec<Vec<cli_table::CellStruct>> = mentees
        .into_iter()
        .map(|mentee| {
            vec![
                mentee.name.cell(),
                mentee.calls.cell().justify(Justify::Right),
            ]
        })
        .collect();

    rows.push(vec![
        "Total".cell().bold(true),
        count.to_string().cell().justify(Justify::Right).bold(true),
    ]);

    let table = rows
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Calls / Month".cell().bold(true),
        ])
        .foreground_color(Some(Color::Green))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}
