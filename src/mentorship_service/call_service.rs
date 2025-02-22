use crate::models::call::Call;
use crate::{constants, error::MenteeError};
use chrono::NaiveDate;
use inquire::{DateSelect, Text};
use rusqlite::{params, Connection};
use std::cell::RefCell;
use std::rc::Rc;

pub struct CallService {
    conn: Rc<RefCell<Connection>>,
}

impl CallService {
    // TODO: change error to a CallError
    pub fn new(conn: Rc<RefCell<Connection>>) -> Result<Self, MenteeError> {
        Ok(Self { conn })
    }

    fn parse_date_from_db(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
    }
}
