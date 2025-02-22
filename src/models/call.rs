#[derive(Debug, Clone)]
pub struct Call {
    pub id: u32,
    pub mentee_id: i64,
    pub date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CallWithMenteeName {
    pub id: u32,
    pub mentee_name: String,
    pub date: String,
    pub notes: Option<String>,
}
