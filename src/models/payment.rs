pub struct Payment {
    pub id: u32,
    pub mentee_id: u32,
    pub date: String,
    pub amount: u32,
    pub mentee_name: Option<String>,
}
