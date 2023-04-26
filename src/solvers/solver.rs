use super::solution::Solution;

pub trait Solver {
    type ConcreteSolution: Solution;
    fn solve(&mut self) -> Self::ConcreteSolution;
}
