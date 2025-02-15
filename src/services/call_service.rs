use rusqlite::Connection;

use crate::repositories::CallRepository;

pub struct CallService<'a> {
    call_repo: CallRepository<'a>,
}

impl<'a> CallService<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self {
            call_repo: CallRepository::new(conn),
        }
    }

    pub fn delete_call(&self, call_id: u32) {
        self.call_repo.delete_call(call_id);

        println!("deleted?")

        // match self.call_repo.delete_call(call_id) {
        //
        // }
        //
        // if deleted > 0 {
        //     Ok(format!("Deleted call with id of {}", call_id.to_string()))
        // } else {
        //     Ok(format!(
        //         "Could not find a call with id of {}",
        //         call_id.to_string()
        //     ))
        // }
    }
}
