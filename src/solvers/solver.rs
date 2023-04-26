use super::vrp_solution::VrpSolution;

pub trait Solver {
    fn solve(&mut self) -> VrpSolution;
}
