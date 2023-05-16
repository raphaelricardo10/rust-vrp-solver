use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceMatrix,
    solvers::{
        solver::Solver, vrp_sequential::vrp_sequential_solver::VrpSequentialSolver, vrp_solution::VrpSolution,
    },
};

use super::greedy_candidate_chooser::GreedyCandidateChooser;

pub struct VrpGreedySolver {
    greedy_solver: VrpSequentialSolver,
}

impl VrpGreedySolver {
    pub fn new(vehicles: Vec<Vehicle>, distances: &DistanceMatrix, stops: Vec<Stop>) -> Self {
        Self {
            greedy_solver: VrpSequentialSolver::new(
                vehicles,
                distances,
                stops,
                Box::new(GreedyCandidateChooser {}),
            ),
        }
    }
}

impl Solver<VrpSolution> for VrpGreedySolver {
    fn solve(&mut self) -> VrpSolution {
        self.greedy_solver.solve()
    }
}
