use std::rc::Rc;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{
        solver::Solver, vrp_sequential::vrp_sequential_solver::VrpSequentialSolver,
        vrp_solution::VrpSolution,
    },
};

use super::greedy_candidate_chooser::GreedyCandidateChooser;

pub struct VrpGreedySolver {
    greedy_solver: VrpSequentialSolver,
}

impl VrpGreedySolver {
    pub fn new(vehicles: Vec<Vehicle>, distances: &DistanceMatrix, stops: Vec<Stop>) -> Self {
        Self {
            greedy_solver: VrpSequentialSolver {
                candidate_chooser: Box::new(GreedyCandidateChooser),
                route_service: RouteService::new(
                    stops.clone(),
                    vehicles,
                    Rc::new(DistanceService::new(stops, distances)),
                ),
            },
        }
    }
}

impl Solver<VrpSolution> for VrpGreedySolver {
    fn solve(&mut self) -> VrpSolution {
        self.greedy_solver.solve()
    }
}
