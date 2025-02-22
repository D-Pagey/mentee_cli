#[derive(Debug, Clone)]
pub struct Video {
    id: u32,
    mentee_id: u32,
    date: String,
    length: u32,
    notes: String,
}

#[derive(Debug, Clone)]
pub struct VideoWithMenteeName {
    pub id: u32,
    pub mentee_name: String,
    pub date: String,
    pub length: u32,
    pub notes: String,
}
