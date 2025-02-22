use rusqlite::{params, Connection};

use crate::constants;

pub struct VideoRepository<'a> {
    conn: &'a Connection,
}

impl<'a> VideoRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    // Delete a video by video id
    pub fn delete_video(&self, video_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::VIDEOS_TABLE);

        self.conn.execute(&sql, params![video_id])
    }
}
