use crate::domain::stop::Stop;

#[derive(Copy, Clone)]
pub struct PathNode<'a> {
    index: usize,
    stop: &'a Stop,
}

impl<'a> PathNode<'a> {
    pub fn new(index: usize, stop: &'a Stop) -> PathNode<'a> {
        PathNode { index, stop }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_stop(&self) -> &Stop {
        self.stop
    }
}
