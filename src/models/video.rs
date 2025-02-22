#[derive(Debug, Clone)]
pub struct Video {
    pub id: i64,
    pub mentee_id: i64,
    pub date: String,
    pub length: u32,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct VideoWithMenteeName {
    pub id: i64,
    pub mentee_name: String,
    pub date: String,
    pub length: u32,
    pub notes: String,
}
