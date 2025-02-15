#[derive(Debug, Clone)]
pub struct Call {
    pub id: u32,
    pub mentee_id: u32,
    pub date: String,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct CallWithMenteeName {
    pub call_id: u32,
    pub mentee_name: String,
    pub date: String,
    pub notes: String,
}
