use chrono::NaiveDate;
use cli_table::{format::Justify, Cell, Color, Style, Table};
use colored::Colorize;

use crate::{
    error::MenteeError,
    mentorship_service::mentee_service::Mentee,
    models::{
        call::CallWithMenteeName,
        mentee::{MenteeWithCounts, Status},
        payment::PaymentWithMenteeName,
        video::VideoWithMenteeName,
    },
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

pub fn format_mentees(mentees: Vec<Mentee>) -> Vec<Vec<String>> {
    let rows: Vec<Vec<String>> = mentees
        .into_iter()
        .map(|mentee| {
            let remaining_calls = mentee
                .remaining_calls
                .map(|count| {
                    if count > 0 {
                        format!("{}", count.to_string().green())
                    } else {
                        format!("{}", count.to_string().red())
                    }
                })
                .unwrap_or_else(|| "".to_string());

            vec![
                capitalize_first_letter_of_each_word(&mentee.name),
                mentee.calls.to_string(),
                remaining_calls,
                capitalize_first_letter_of_each_word(Status::as_str(&mentee.status)),
                mentee.notes,
            ]
        })
        .collect();

    rows
}

fn format_date(date_str: &str) -> Result<String, chrono::ParseError> {
    // Parse the date string (ISO 8601 format)
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    // Format the date to "day, month, year"
    Ok(date.format("%d %b %Y").to_string())
}

pub fn format_videos(videos: Vec<VideoWithMenteeName>) -> Vec<Vec<String>> {
    let rows: Vec<Vec<String>> = videos
        .into_iter()
        .map(|video| {
            let formatted_date = format_date(&video.date).unwrap_or_else(|_| video.date.clone());

            vec![
                video.id.to_string(),
                capitalize_first_letter_of_each_word(&video.mentee_name),
                formatted_date,
                video.length.to_string(),
                video.notes,
            ]
        })
        .collect();

    rows
}
pub fn format_calls(calls: Vec<CallWithMenteeName>) -> Vec<Vec<String>> {
    let rows: Vec<Vec<String>> = calls
        .into_iter()
        .map(|call| {
            let formatted_date = format_date(&call.date).unwrap_or_else(|_| call.date.clone());

            vec![
                call.id.to_string(),
                capitalize_first_letter_of_each_word(&call.mentee_name),
                formatted_date,
                call.notes.unwrap_or("".to_string()),
            ]
        })
        .collect();

    rows
}

pub fn format_payments(payments: Vec<PaymentWithMenteeName>) -> Vec<Vec<String>> {
    let rows: Vec<Vec<String>> = payments
        .into_iter()
        .map(|payment| {
            let formatted_date =
                format_date(&payment.date).unwrap_or_else(|_| payment.date.clone());

            vec![
                payment.id.to_string(),
                capitalize_first_letter_of_each_word(&payment.mentee_name),
                formatted_date,
                payment.amount.to_string(),
            ]
        })
        .collect();

    rows
}

pub fn render_mentees_table(mentees: Vec<Mentee>) -> Result<(), MenteeError> {
    let rows = format_mentees(mentees);

    let cell_rows: Vec<Vec<cli_table::CellStruct>> = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.cell().justify(Justify::Right))
                .collect()
        })
        .collect();

    let table = cell_rows
        .table()
        .title(vec![
            "Name".cell().bold(true),
            "Calls / Month".cell().bold(true),
            "Remaining Calls".cell().bold(true),
            "Status".cell().bold(true),
            "Notes".cell().bold(true),
        ])
        .foreground_color(Some(Color::Blue))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}

pub fn render_calls_table(calls: Vec<CallWithMenteeName>) -> Result<(), MenteeError> {
    let rows = format_calls(calls);

    let cell_rows: Vec<Vec<cli_table::CellStruct>> = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.cell().justify(Justify::Right))
                .collect()
        })
        .collect();

    let table = cell_rows
        .table()
        .title(vec![
            "Call Id".cell().bold(true),
            "Mentee".cell().bold(true),
            "Date".cell().bold(true),
            "Notes".cell().bold(true),
        ])
        .foreground_color(Some(Color::Yellow))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}

pub fn render_videos_table(videos: Vec<VideoWithMenteeName>) -> Result<(), MenteeError> {
    let rows = format_videos(videos);

    let cell_rows: Vec<Vec<cli_table::CellStruct>> = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.cell().justify(Justify::Right))
                .collect()
        })
        .collect();

    let table = cell_rows
        .table()
        .title(vec![
            "Video Id".cell().bold(true),
            "Mentee".cell().bold(true),
            "Date".cell().bold(true),
            "Length".cell().bold(true),
            "Notes".cell().bold(true),
        ])
        .foreground_color(Some(Color::Magenta))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}

pub fn render_payments_table(payments: Vec<PaymentWithMenteeName>) -> Result<(), MenteeError> {
    let rows = format_payments(payments);

    let cell_rows: Vec<Vec<cli_table::CellStruct>> = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.cell().justify(Justify::Right))
                .collect()
        })
        .collect();

    let table = cell_rows
        .table()
        .title(vec![
            "Payment Id".cell().bold(true),
            "Mentee".cell().bold(true),
            "Date".cell().bold(true),
            "Amount".cell().bold(true),
        ])
        .foreground_color(Some(Color::Green))
        .bold(true);

    let table_display = table.display()?;

    Ok(println!("{}", table_display))
}

pub fn display_mentee(mentee: MenteeWithCounts) {
    println!("\nMentee Details:");
    println!("-----------------------");
    println!(
        "Name:             {}",
        capitalize_first_letter_of_each_word(&mentee.mentee.name)
    );
    println!("Status:           {:?}", mentee.mentee.status);
    println!(
        "Payment Day:      {}",
        add_ordinal_suffix(mentee.mentee.payment_day)
    );

    println!("\nPayment Details:");
    println!("-----------------------");
    println!("Gross:            ${:.2}", mentee.mentee.gross);
    println!("Net:              ${:.2}", mentee.mentee.net);

    let net_per_call = calc_net_per_call(&mentee.mentee.net, &mentee.mentee.calls);
    println!("Net / Call:       ${:.2}", net_per_call);
    println!("Total Payments:   {}", mentee.payment_count);

    println!("\nCall Details:");
    println!("-----------------------");
    println!("Calls / Month:    {}", mentee.mentee.calls);
    println!("Total Calls:      {}", mentee.call_count);
    println!("Total Videos:     {}", mentee.video_count);
    let remaining_calls = mentee.remaining_calls;
    let remaining_calls_colored = if remaining_calls > 0 {
        remaining_calls.to_string().green()
    } else {
        remaining_calls.to_string().red()
    };
    println!("Remaining Calls:  {}", remaining_calls_colored);

    println!(
        "Notes:            {}",
        mentee.mentee.notes.unwrap_or("".to_string())
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_mentees() {
        let mentees = vec![Mentee {
            name: "john doe".to_string(),
            calls: 10,
            gross: 1000,
            net: 900,
            status: Status::Warm,
            payment_day: 5,
            notes: "CET timezone".to_string(),
            call_count: Some(10),
            payment_count: Some(0),
            video_count: Some(0),
            remaining_calls: Some(0),
        }];

        let rows = format_mentees(mentees);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], "John Doe");
        assert_eq!(rows[0][1], "10");
        assert_eq!(rows[0][2], "1000");
        assert_eq!(rows[0][3], "900");
        assert_eq!(rows[0][4], "90");
        assert_eq!(rows[0][5], "Warm");
        assert_eq!(rows[0][6], "5th");
        assert_eq!(rows[0][7], "CET timezone");
    }

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

    #[test]
    fn capitalize_one_word() {
        let result = capitalize_first_letter_of_each_word("dan");
        assert_eq!(result, "Dan")
    }

    #[test]
    fn capitalize_multiple_words() {
        let result = capitalize_first_letter_of_each_word("dan page");
        assert_eq!(result, "Dan Page")
    }
}
