use crate::constants;
use crate::mentee::Mentee;

use inquire::Text;
use rusqlite::{Connection, Result};

pub struct MenteeService {
    conn: Connection,
}

impl MenteeService {
    pub fn new(database_url: &str) -> Result<Self> {
        let conn = Connection::open(database_url)?;

        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        calls INTEGER
        )",
            constants::MENTEE_TABLE
        );

        conn.execute(&sql, ())?;

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
            &format!(
                "INSERT INTO {} (name, calls) VALUES (?1, ?2)",
                constants::MENTEE_TABLE
            ),
            (&mentee.name, &mentee.calls),
        )?;

        Ok(mentee)
    }

    pub fn delete_mentee(&self, name: String) {
        match self.conn.execute(
            &format!("DELETE FROM {} WHERE name = :name", constants::MENTEE_TABLE),
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
            &format!(
                "UPDATE {} SET calls = ?1 WHERE name = ?2",
                constants::MENTEE_TABLE
            ),
            (&calls, &name),
        ) {
            Ok(updated) => println!("updated...{updated}"),
            Err(error) => println!("you fucked up - {error}"),
        }
    }

    pub fn get_all_mentees(&self) -> Result<Vec<Mentee>> {
        let sql = format!("SELECT name, calls FROM {}", constants::MENTEE_TABLE);
        let mut stmt = self.conn.prepare(&sql)?;
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

        Ok(mentees)
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
