use super::solution::Solution;

pub trait Solver<T: Solution> {
    fn solve(&mut self) -> T;
}

pub trait SolverCallbacks {
    fn before_solving(&mut self) {}
    fn after_solving(&mut self) {}
}
