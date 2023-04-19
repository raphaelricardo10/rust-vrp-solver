use crate::domain::stop::Stop;

#[derive(Copy, Clone)]
pub struct Neighbor<'a> {
    pub index: usize,
    pub stop: &'a Stop,
}

impl<'a> Neighbor<'a> {
    pub fn new(index: usize, stop: &'a Stop) -> Neighbor<'a> {
        Neighbor { index, stop }
    }
}
