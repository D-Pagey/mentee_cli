use cli_table::{format::Justify, Cell, Color, Style, Table};

use crate::{
    error::MenteeError,
    mentee::{Mentee, Status},
};

pub fn render_mentees_table(mentees: Vec<Mentee>) -> Result<(), MenteeError> {
    let rows: Vec<Vec<cli_table::CellStruct>> = mentees
        .into_iter()
        .map(|mentee| {
            vec![
                mentee.name.cell(),
                mentee.calls.cell().justify(Justify::Right),
                mentee.gross.cell().justify(Justify::Right),
                mentee.net.cell().justify(Justify::Right),
                Status::as_str(&mentee.status)
                    .cell()
                    .justify(Justify::Right),
                mentee.payment_day.cell().justify(Justify::Right),
            ]
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Calls / Month".cell().bold(true),
            "Gross".cell().bold(true),
            "Net".cell().bold(true),
            "Status".cell().bold(true),
            "Payment Day".cell().bold(true),
        ])
        .foreground_color(Some(Color::Green))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}
