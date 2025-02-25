use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Mentee {
    #[allow(dead_code)]
    pub id: u32,
    pub name: String,
    pub calls: u32,
    pub status: Status,
    pub gross: u32,
    pub net: u32,
    pub payment_day: u32,
    pub notes: Option<String>,
}

#[allow(dead_code)]
pub struct MenteeWithCounts {
    pub mentee: Mentee,
    pub call_count: i64,
    pub payment_count: i64,
    pub video_count: i64,
    pub remaining_calls: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_as_str() {
        assert_eq!(Status::Hot.as_str(), "hot");
        assert_eq!(Status::Warm.as_str(), "warm");
        assert_eq!(Status::Cold.as_str(), "cold");
        assert_eq!(Status::Archived.as_str(), "archived");
    }

    #[test]
    fn test_status_from_str() {
        assert_eq!(Status::from_str("hot"), Some(Status::Hot));
        assert_eq!(Status::from_str("warm"), Some(Status::Warm));
        assert_eq!(Status::from_str("cold"), Some(Status::Cold));
        assert_eq!(Status::from_str("archived"), Some(Status::Archived));

        assert_eq!(Status::from_str("unknown"), None);
        assert_eq!(Status::from_str(""), None);
        assert_eq!(Status::from_str("_"), None);
        assert_eq!(Status::from_str("hott"), None);
    }

    #[test]
    fn test_status_variants() {
        let expected = vec!["hot", "warm", "cold", "archived"];
        assert_eq!(Status::variants(), expected);
    }
}
