use rusqlite::Connection;

use crate::{
    error::MenteeError,
    models::call::CallWithMenteeName,
    repositories::{mentee_repository::MenteeRepository, CallRepository},
};

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

    pub fn get_all_calls(
        &self,
        name: Option<String>,
    ) -> Result<Vec<CallWithMenteeName>, MenteeError> {
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

        self.call_repo
            .get_all_calls(mentee_id)
            .map_err(MenteeError::DatabaseError)
    }

    pub fn delete_call(&self, call_id: u32) -> Result<String, String> {
        match self.call_repo.delete_call(call_id) {
            Ok(0) => Err(format!("Call with ID {} not found.", call_id)),
            Ok(_) => Ok(format!("Deleted call {}", call_id)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
}
