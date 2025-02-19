use rusqlite::Connection;

use crate::repositories::{mentee_repository::MenteeRepository, CallRepository};

pub struct CallService<'a> {
    call_repo: CallRepository<'a>,
    mentee_repo: MenteeRepository<'a>,
}

impl<'a> CallService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            call_repo: CallRepository::new(conn),
            mentee_repo: MenteeRepository::new(conn),
        }
    }

    pub fn delete_call(&self, call_id: u32) -> Result<String, String> {
        match self.call_repo.delete_call(call_id) {
            Ok(0) => Err(format!("Call with ID {} not found.", call_id)),
            Ok(_) => Ok(format!("Deleted call {}", call_id)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
}
