use crate::{constants, error::MenteeError};
use inquire::{DateSelect, Text};
use rusqlite::{Connection, OptionalExtension};
use std::cell::RefCell;
use std::rc::Rc;

struct Video {
    id: u32,
    mentee_id: u32,
    date: String,
    length: u32,
    notes: String,
}

pub struct VideoWithMenteeName {
    pub id: u32,
    pub mentee_name: String,
    pub date: String,
    pub length: u32,
    pub notes: String,
}

pub struct VideoService {
    conn: Rc<RefCell<Connection>>,
}

impl VideoService {
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mentee_id INTEGER NOT NULL,
            date TEST NOT NULL,
            length INTEGER NOT NULL,
            notes TEXT,
            FOREIGN KEY (mentee_id) REFERENCES {} (id))",
            constants::VIDEOS_TABLE,
            constants::MENTEES_TABLE
        );

        conn.borrow().execute(&sql, ())?;

        Ok(Self { conn })
    }

    fn get_mentee_id(&self, name: &str) -> Result<Option<i64>, rusqlite::Error> {
        let sql = format!(
            "SELECT id FROM {} WHERE name = ? LIMIT 1",
            constants::MENTEES_TABLE,
        );

        self.conn
            .borrow()
            .query_row(&sql, &[name], |row| row.get(0))
            .optional()
    }

    pub fn get_all_videos(
        &self,
        name: Option<String>,
    ) -> Result<Vec<VideoWithMenteeName>, MenteeError> {
        let mut sql = format!(
            "
        SELECT
            videos.id AS video_id,
            mentees.name AS mentee_name,
            videos.date,
            videos.length,
            videos.notes
        FROM
            {}
        JOIN
            {}
        ON
            videos.mentee_id = mentees.id
        ",
            constants::VIDEOS_TABLE,
            constants::MENTEES_TABLE
        );

        if let Some(name) = name {
            let mentee_id = match self.get_mentee_id(&name)? {
                Some(id) => id,
                None => {
                    // TODO: change this to error not OK
                    println!("No mentee found with the name '{}'.", name);
                    return Ok(vec![]); // Return early with an empty vector
                }
            };

            sql.push_str(format!("WHERE videos.mentee_id = {} ", &mentee_id).as_str());
        }

        sql.push_str("ORDER BY videos.date DESC");

        let binding = self.conn.borrow();
        let mut stmt = binding.prepare(&sql)?;

        let video_iter = stmt.query_map([], |row| {
            Ok(VideoWithMenteeName {
                id: row.get(0)?,
                mentee_name: row.get(1)?,
                date: row.get(2)?,
                length: row.get(3)?,
                notes: row.get(4)?,
            })
        })?;

        let mut videos: Vec<VideoWithMenteeName> = Vec::new();

        for video_result in video_iter {
            videos.push(video_result?)
        }

        Ok(videos)
    }
}
