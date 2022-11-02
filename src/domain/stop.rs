pub struct Stop {
    id: u32,
    pub usage: u32,
}

impl Stop {
    pub fn new(id: u32) -> Stop {
        Stop { id, usage: 0 }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
