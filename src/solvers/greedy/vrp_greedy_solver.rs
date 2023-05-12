use std::rc::Rc;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{
        sequential_solver::{SequentialSolver, SequentialSolverParameters},
        solver::SolverCallbacks,
        vrp_solution::VrpSolution,
    },
};

use super::greedy_solver::GreedySolver;

pub struct VrpGreedySolver {
    route_service: RouteService,
}

impl GreedySolver for VrpGreedySolver {}

impl SolverCallbacks for VrpGreedySolver {
    fn before_solving(&mut self) {
        self.route_service.assign_starting_points();
    }

    fn after_solving(&mut self) {
        self.route_service.assign_stop_points();
    }
}

impl SequentialSolverParameters for VrpGreedySolver {
    type SequenceId = u32;
    type CandidateId = u32;
    type Cost = f32;
}

impl SequentialSolver<VrpSolution, VrpGreedySolver> for VrpGreedySolver {
    fn get_solution(&self) -> VrpSolution {
        VrpSolution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        )
    }

    fn stop_condition_met(&self) -> bool {
        !self.route_service.has_available_stop()
    }

    fn choose_candidate(&mut self, sequence_id: u32, candidate_id: u32) {
        self.route_service
            .assign_stop_to_route(sequence_id, candidate_id)
            .unwrap_or_else(|_| {
                panic!("the vehicle {sequence_id} should support the load from {candidate_id}")
            });
    }

    fn get_all_sequences(&self) -> Box<dyn Iterator<Item = u32> + '_> {
        Box::new(self.route_service.get_all_routes().keys().cloned())
    }

    fn get_all_candidates(&self, sequence_id: u32) -> Box<dyn Iterator<Item = (u32, f32)> + '_> {
        Box::new(self.route_service.get_distances_from(sequence_id))
    }
}

impl VrpGreedySolver {
    pub fn new(
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        stops: Vec<Stop>,
    ) -> VrpGreedySolver {
        VrpGreedySolver {
            route_service: RouteService::new(
                stops.clone(),
                vehicles,
                Rc::new(DistanceService::new(stops, distances)),
            ),
        }
    }
}
