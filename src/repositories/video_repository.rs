use rusqlite::{params, Connection};

use crate::{constants, models::video::VideoWithMenteeName};

pub struct VideoRepository<'a> {
    conn: &'a Connection,
}

impl<'a> VideoRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_all_videos(
        &self,
        mentee_id: Option<i64>,
    ) -> Result<Vec<VideoWithMenteeName>, rusqlite::Error> {
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
        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
        let id_storage;

        if let Some(id) = mentee_id {
            sql.push_str(" WHERE videos.mentee_id = ?1");
            id_storage = id;
            params.push(&id_storage);
        }

        sql.push_str(" ORDER BY videos.date DESC");

        let mut stmt = self.conn.prepare(&sql)?;
        let video_iter = stmt.query_map(&params[..], |row| {
            Ok(VideoWithMenteeName {
                id: row.get(0)?,
                mentee_name: row.get(1)?,
                date: row.get(2)?,
                length: row.get(3)?,
                notes: row.get(4)?,
            })
        })?;

        let mut videos = Vec::new();
        for video in video_iter {
            videos.push(video?);
        }

        Ok(videos)
    }

    // Delete a video by video id
    pub fn delete_video(&self, video_id: u32) -> Result<usize, rusqlite::Error> {
        let sql = format!("DELETE FROM {} WHERE id = ?1", constants::VIDEOS_TABLE);

        self.conn.execute(&sql, params![video_id])
    }
}
