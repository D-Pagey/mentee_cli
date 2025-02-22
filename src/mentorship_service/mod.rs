pub mod call_service;
pub mod mentee_service;
pub mod payment_service;
pub mod video_service;

use dirs::home_dir;
use mentee_service::MenteeService;
use payment_service::PaymentService;
use rusqlite::Connection;
use std::cell::RefCell;
use std::rc::Rc;
use video_service::VideoService;

use crate::error::MenteeError;

pub struct MentorshipService {
    pub mentee_service: MenteeService,
    pub payment_service: PaymentService,
    pub video_service: VideoService,
}

impl MentorshipService {
    pub fn new() -> Result<Self, MenteeError> {
        // Get the user's home directory
        let mut db_path = home_dir().ok_or(MenteeError::HomeDirNotFound)?;
        db_path.push(".mentees"); // Directory to store db
        std::fs::create_dir_all(&db_path)?; // Ensure directory exists

        if cfg!(debug_assertions) {
            // Dev database path
            db_path.push("mentees_dev.db");
        } else {
            // Production database path
            db_path.push("mentees.db");
        }

        // Wrap the connection so sub‐services can clone it
        let conn = Rc::new(RefCell::new(Connection::open(db_path)?));

        let mentee_service = MenteeService::new(conn.clone())?;
        let payment_service = PaymentService::new(conn.clone())?;
        let video_service = VideoService::new(conn.clone())?;

        Ok(Self {
            mentee_service,
            payment_service,
            video_service,
        })
    }
}

// let mut db_path = if test_mode {
//     std::env::temp_dir()
// } else {
//     home_dir().ok_or(MenteeError::HomeDirNotFound)?
// };
