use cli_table::{format::Justify, Cell, Color, Style, Table};

use crate::{
    error::MenteeError,
    mentee::{Mentee, Status},
};

fn calc_net_per_call(net: &u32, calls: &u32) -> u32 {
    if *calls == 0 {
        *net
    } else {
        net / calls
    }
}

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
            let net_per_call = calc_net_per_call(&mentee.net, &mentee.calls);

            vec![
                capitalize_first_letter_of_each_word(&mentee.name).cell(),
                mentee.calls.cell().justify(Justify::Right),
                mentee.gross.cell().justify(Justify::Right),
                mentee.net.cell().justify(Justify::Right),
                net_per_call.cell().justify(Justify::Right),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_for_zero_calls() {
        let net_amount = 200;
        let result = calc_net_per_call(&net_amount, &0);
        assert_eq!(result, net_amount)
    }

    #[test]
    fn correct_net_per_call() {
        let net_amount = 200;
        let result = calc_net_per_call(&net_amount, &2);
        assert_eq!(result, 100)
    }

    #[test]
    fn correct_suffix_for_day() {
        let test_cases = vec![
            (11, "11th"),
            (12, "12th"),
            (13, "13th"),
            (1, "1st"),
            (33, "33rd"),
            (22, "22nd"),
            (101, "101st"),
            (112, "112th"),
        ];

        for (input, expected) in test_cases {
            let result = add_ordinal_suffix(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
