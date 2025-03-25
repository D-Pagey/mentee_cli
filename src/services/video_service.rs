use colored::Colorize;
use inquire::{CustomType, DateSelect, Text};
use rusqlite::Connection;

use crate::{
    error::MenteeError,
    models::video::{Video, VideoWithMenteeName},
    repositories::{MenteeRepository, VideoRepository},
    utils::validation::parse_date_from_db,
};

pub struct VideoService<'a> {
    mentee_repo: MenteeRepository<'a>,
    video_repo: VideoRepository<'a>,
}

impl<'a> VideoService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            mentee_repo: MenteeRepository::new(conn),
            video_repo: VideoRepository::new(conn),
        }
    }

    pub fn add_video(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = self
            .mentee_repo
            .get_mentee_id(&name)?
            .ok_or_else(|| MenteeError::NotFound(format!("No mentee with name '{}'", name)))?;

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

        let result = self.video_repo.add_video(Video {
            id: 0,
            mentee_id,
            date: date.clone(),
            length,
            notes,
        });

        match result {
            Ok(_) => Ok(format!("Video log with {name} on {date} added.")
                .green()
                .to_string()),
            Err(err) => Err(MenteeError::from(err)),
        }
    }

    pub fn get_all_videos(
        &self,
        name: Option<String>,
    ) -> Result<Vec<VideoWithMenteeName>, MenteeError> {
        let mentee_id = if let Some(name) = name {
            match self.mentee_repo.get_mentee_id(&name)? {
                Some(id) => Some(id),
                None => {
                    return Err(MenteeError::NotFound(format!(
                        "No mentee found with name '{}'",
                        name
                    )))
                }
            }
        } else {
            None
        };

        self.video_repo
            .get_all_videos(mentee_id)
            .map_err(MenteeError::DatabaseError)
    }

    pub fn update_video(&self, video_id: u32) -> Result<String, MenteeError> {
        let video = self
            .video_repo
            .get_video_by_id(video_id)
            .map_err(|_| MenteeError::NotFound(format!("Video with id of {}", video_id)))?;

        let parsed_date = parse_date_from_db(&video.date).unwrap();

        let date = DateSelect::new("Enter the date of the video:")
            .with_default(parsed_date)
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

        let updated = self
            .video_repo
            .update_video(date, length, notes, video_id)?;

        Ok(format!("{updated} video record updated"))
    }

    pub fn delete_video(&self, video_id: u32) -> Result<String, MenteeError> {
        let result = self.video_repo.delete_video(video_id);

        match result {
            Ok(0) => Err(MenteeError::NotFound(format!("Video with Id {}", video_id))),
            Ok(_) => Ok(format!("Video with Id of {} deleted.", video_id)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }
}
