use cli_table::{format::Justify, Cell, Color, Style, Table};

use crate::{
    error::MenteeError,
    mentee::{Mentee, Status},
};

fn add_ordinal_suffix(n: u32) -> String {
    let suffix = match n % 100 {
        11 | 12 | 13 => "th", // Special case for 11, 12, 13
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{}", n, suffix)
}

fn capitalize_first_letter_of_each_word(s: &str) -> String {
    s.split_whitespace() // Split the string by whitespace
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(), // Handle empty words
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<String>>() // Collect capitalized words into a vector
        .join(" ") // Join words with a space
}

pub fn render_mentees_table(mentees: Vec<Mentee>) -> Result<(), MenteeError> {
    let rows: Vec<Vec<cli_table::CellStruct>> = mentees
        .into_iter()
        .map(|mentee| {
            vec![
                capitalize_first_letter_of_each_word(&mentee.name).cell(),
                mentee.calls.cell().justify(Justify::Right),
                mentee.gross.cell().justify(Justify::Right),
                mentee.net.cell().justify(Justify::Right),
                mentee.net_per_call.cell().justify(Justify::Right),
                capitalize_first_letter_of_each_word(Status::as_str(&mentee.status))
                    .cell()
                    .justify(Justify::Right),
                add_ordinal_suffix(mentee.payment_day)
                    .cell()
                    .justify(Justify::Right),
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
            "Net / Call".cell().bold(true),
            "Status".cell().bold(true),
            "Payment Day".cell().bold(true),
        ])
        .foreground_color(Some(Color::Blue))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}
