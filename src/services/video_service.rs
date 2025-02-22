use rusqlite::Connection;

use crate::{error::MenteeError, repositories::VideoRepository};

pub struct VideoService<'a> {
    video_repo: VideoRepository<'a>,
}

impl<'a> VideoService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            video_repo: VideoRepository::new(conn),
        }
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
