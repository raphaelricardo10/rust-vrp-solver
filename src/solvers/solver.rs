use super::solution::Solution;

pub trait Solver<T: Solution> {
    fn solve(&mut self) -> T;
}
