use rusqlite::Connection;

use crate::{
    error::MenteeError,
    models::video::VideoWithMenteeName,
    repositories::{MenteeRepository, VideoRepository},
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

    pub fn delete_video(&self, video_id: u32) -> Result<String, MenteeError> {
        let result = self.video_repo.delete_video(video_id);

        match result {
            Ok(0) => Err(MenteeError::NotFound(format!("Video with Id {}", video_id))),
            Ok(_) => Ok(format!("Video with Id of {} deleted.", video_id)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }
}
