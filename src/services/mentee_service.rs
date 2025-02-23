use rusqlite::Connection;

use crate::{error::MenteeError, repositories::MenteeRepository};

pub struct MenteeService<'a> {
    mentee_repo: MenteeRepository<'a>,
}

impl<'a> MenteeService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            mentee_repo: MenteeRepository::new(conn),
        }
    }

    // TODO: handle cascade deletes
    pub fn delete_mentee(&self, name: String) -> Result<String, MenteeError> {
        let mentee_id = self.mentee_repo.get_mentee_id(&name)?.ok_or_else(|| {
            MenteeError::NotFound(format!("No mentee found with name '{}'", name))
        })?;

        match self.mentee_repo.delete_mentee_by_id(mentee_id) {
            Ok(_) => Ok(format!("Deleted mentee {}", name)),
            Err(err) => Err(MenteeError::DatabaseError(err)),
        }
    }
}
