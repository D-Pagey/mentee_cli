use crate::{constants, error::MenteeError};
use chrono::NaiveDate;
use inquire::{CustomType, DateSelect, Text};
use rusqlite::{params, Connection, OptionalExtension};
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

    pub fn add_video(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = match self.get_mentee_id(&name)? {
            Some(id) => id,
            None => return Ok(format!("No mentee found with the name '{}'.", name)),
        };

        let date = DateSelect::new("Enter the date of the video:")
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let length = inquire::prompt_u32("Roughly how long was the video?")?;

        let notes = Text::new("Enter any notes for the video:")
            .with_placeholder("e.g. Discussed project progress ")
            .prompt()
            .expect("Failed to read notes");

        let result = self.conn.borrow().execute(
            &format!(
                "INSERT INTO {} (mentee_id, date, length, notes) VALUES (?1, ?2, ?3, ?4)",
                constants::VIDEOS_TABLE
            ),
            (&mentee_id, &date, &length, &notes),
        );

        match result {
            Ok(_) => Ok(format!("Video log with {name} on {date} added.")),
            Err(err) => Err(MenteeError::from(err)),
        }
    }

    fn parse_date_from_db(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
    }

    pub fn update_video(&self, video_id: u32) -> Result<String, MenteeError> {
        let sql = format!("SELECT * FROM {} WHERE id = ?1", constants::VIDEOS_TABLE);

        let video_result = self
            .conn
            .borrow()
            .query_row(&sql, params![video_id], |row| {
                Ok(Video {
                    id: row.get(0)?,
                    mentee_id: row.get(1)?,
                    date: row.get(2)?,
                    length: row.get(3)?,
                    notes: row.get(4)?,
                })
            });

        let video = match video_result {
            Ok(video) => video,
            _ => return Ok(format!("Can't find a video with id of {}", video_id)),
        };

        // TODO: deal with this
        let parsed = VideoService::parse_date_from_db(&video.date).unwrap();

        let date = DateSelect::new("Enter the date of the video:")
            .with_default(parsed)
            .prompt()
            .expect("Failed to read date")
            .format("%Y-%m-%d")
            .to_string();

        let length: u32 = CustomType::new("Roughly how long was the video?")
            .with_starting_input(&video.length.to_string())
            .prompt()
            .expect("Failed to read length");

        let notes = Text::new("Enter any notes for the video:")
            .with_placeholder("e.g. Discussed project progress ")
            .with_initial_value(&video.notes)
            .prompt()
            .expect("Failed to read notes");

        let update_sql = format!(
            "UPDATE {} SET date = ?1, length = ?2, notes = ?3 WHERE id = ?4",
            constants::VIDEOS_TABLE
        );

        let result = self
            .conn
            .borrow()
            .execute(&update_sql, params![date, length, notes, video_id])?;

        Ok(format!("{result} video record updated"))
    }
}
