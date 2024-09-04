use crate::constants;
use crate::mentee::Mentee;

use cli_table::{format::Justify, Cell, Style, Table};
use inquire::Text;
use rusqlite::{Connection, Result};

pub struct MenteeService {
    conn: Connection,
}

impl MenteeService {
    pub fn new(database_url: &str) -> Result<Self> {
        let conn = Connection::open(database_url)?;

        println!("{}", constants::MENTEE_TABLE);

        // TODO: make us of the public const for table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS mentees (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        calls INTEGER
    )",
            (),
        )?;

        Ok(MenteeService { conn })
    }

    pub fn add_mentee(&self) -> Result<Mentee> {
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

        self.conn.execute(
            "INSERT INTO mentees (name, calls) VALUES (?1, ?2)",
            (&mentee.name, &mentee.calls),
        )?;

        Ok(mentee)
    }

    pub fn delete_mentee(&self, name: String) {
        match self.conn.execute(
            "DELETE FROM mentees WHERE name = :name",
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

    pub fn update_mentee(&self, name: String) {
        let calls = Text::new("How many calls per month do they have?")
            .prompt()
            .expect("Failed to capture mentee name");

        match self.conn.execute(
            "UPDATE mentees SET calls = ?1 WHERE name = ?2",
            (&calls, &name),
        ) {
            Ok(updated) => println!("updated...{updated}"),
            Err(error) => println!("you fucked up - {error}"),
        }
    }

    pub fn get_all_mentees(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT name, calls FROM mentees")?;
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
}

// impl Mentee {
//     // TODO: remove unwraps, deal with ? for conn.execute, add error to result type
//     pub fn add(conn: &Connection) -> Result<(), Box<dyn Error>> {
//         // match name {
//         //     Ok(name) => println!("Your name is being published...{}", name),
//         //     Err(err) => println!("Error while publishing...{}", err),
//         // }
//
//         // TODO: add validator to parse to number then check max calls
//
//         // how to use the parsing_u32
//         // let calls = Text::new("How many calls per month do they have?").prom
//
//         // match calls {
//         //     Ok(calls) => println!("Your calls is being published...{}", calls),
//         //     Err(err) => println!("Error while publishing...{}", err),
//         // }
//         // ::build vs ::new
//         // the struct implementation validates the number of calls
//         // returns valid error message i.e too many calls
//         //
//         //
//
//         Ok(())
//     }
// }
