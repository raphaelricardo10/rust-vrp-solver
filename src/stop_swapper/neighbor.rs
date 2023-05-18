use crate::domain::stop::Stop;

#[derive(Copy, Clone)]
pub struct Neighbor {
    pub index: usize,
    pub stop: Stop,
}
