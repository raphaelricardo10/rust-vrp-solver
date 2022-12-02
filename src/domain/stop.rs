use std::hash::Hash;
#[derive(Copy, Clone, Eq)]
pub struct Stop {
    pub id: u32,
    pub usage: u32,
}

impl PartialEq for Stop {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Stop {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Stop {
    pub fn new(id: u32, usage: u32) -> Stop {
        Stop { id, usage }
    }
}
