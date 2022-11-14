#[derive(Copy, Clone)]
pub struct Stop {
    pub id: u32,
    pub usage: u32,
}

impl Stop {
    pub fn new(id: u32, usage: u32) -> Stop {
        Stop { id, usage }
    }
}
