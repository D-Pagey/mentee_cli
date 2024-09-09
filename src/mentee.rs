use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Status {
    Archived,
    Cold,
    Warm,
    Hot,
}

impl Status {
    // convert enum variant to a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Hot => "hot",
            Status::Warm => "warm",
            Status::Cold => "cold",
            Status::Archived => "archived",
        }
    }

    pub fn from_str(s: &str) -> Option<Status> {
        match s {
            "hot" => Some(Status::Hot),
            "warm" => Some(Status::Warm),
            "cold" => Some(Status::Cold),
            "archived" => Some(Status::Archived),
            _ => None,
        }
    }

    pub fn variants() -> Vec<&'static str> {
        vec![
            Status::Hot.as_str(),
            Status::Warm.as_str(),
            Status::Cold.as_str(),
            Status::Archived.as_str(),
        ]
    }
}

pub struct Mentee {
    pub name: String,
    pub calls: u32,
    pub status: Status,
    pub gross: u32,
    pub net: u32,
    pub payment_day: u32, // TODO: enum between 1 and 31?
}

// TODO: should i validate here? for adding and updating?
// impl Mentee {
//     pub fn new(id: u32, name: String, calls: u32) -> Self {
//         Mentee { id, name, calls }
//     }
// }
