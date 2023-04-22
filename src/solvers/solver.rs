use super::solution::Solution;

pub trait Solver {
    fn solve(&mut self);
    fn get_solution(&self) -> &Solution;
}
