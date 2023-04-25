use super::solution::Solution;

pub trait Solver {
    fn solve(&mut self) -> Solution;
}
