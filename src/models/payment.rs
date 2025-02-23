pub struct Payment {
    #[allow(dead_code)]
    pub id: u32,
    #[allow(dead_code)]
    pub mentee_id: u32,
    pub date: String,
    pub amount: u32,
}

pub struct PaymentWithMenteeName {
    pub id: u32,
    pub mentee_name: String,
    pub date: String,
    pub amount: u32,
}
