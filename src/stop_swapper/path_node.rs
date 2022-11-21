use crate::domain::stop::Stop;

#[derive(Copy, Clone)]
pub struct PathNode<'a> {
    pub index: usize,
    pub stop: &'a Stop,
}

impl<'a> PathNode<'a> {
    pub fn new(index: usize, stop: &'a Stop) -> PathNode<'a> {
        PathNode { index, stop }
    }
}
