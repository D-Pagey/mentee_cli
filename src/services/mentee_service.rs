use rusqlite::Connection;

use crate::repositories::MenteeRepository;

pub struct MenteeService<'a> {
    mentee_repo: MenteeRepository<'a>,
}

impl<'a> MenteeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            mentee_repo: MenteeRepository::new(conn),
        }
    }
}
