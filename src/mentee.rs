enum Status {
    Archived,
    Cold,
    Warm,
    Hot,
}

pub struct Mentee {
    // pub id: u32,
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
