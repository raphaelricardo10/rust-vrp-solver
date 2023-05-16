use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    local_search::two_opt::TwoOptSearcher,
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{
        solver::Solver, vrp_sequential::vrp_sequential_solver::VrpSequentialSolver,
        vrp_solution::VrpSolution,
    },
};

use super::{
    grasp_candidate_chooser::GraspCandidateChooser,
    grasp_solver::{GraspSolver, GraspSolverCallbacks, SolverWithGraspCallbacks},
};

#[repr(C)]
pub struct GraspSolverParameters {
    pub rcl_size: usize,
    pub max_improvement_times: u8,
}

pub struct VrpGraspSolver {
    grasp_solver: GraspSolver<VrpSolution>,
}

impl Solver<VrpSolution> for VrpGraspSolver {
    fn solve(&mut self) -> VrpSolution {
        self.grasp_solver.solve()
    }
}

impl GraspSolverCallbacks for VrpSequentialSolver {
    fn after_iteration(&mut self) {
        self.route_service.reset();
    }
}

impl SolverWithGraspCallbacks<VrpSolution> for VrpSequentialSolver {}

impl VrpGraspSolver {
    pub fn new<R: Rng + 'static>(
        stops: Vec<Stop>,
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        parameters: GraspSolverParameters,
        rng: R,
    ) -> Self {
        let distance_service = Rc::new(DistanceService::new(stops.clone(), distances));

        let candidate_chooser = Box::new(GraspCandidateChooser {
            rcl_size: parameters.rcl_size,
            rng: RefCell::new(rng),
        });

        let route_service = RouteService::new(stops, vehicles, distance_service.clone());

        let first_stage_solver = VrpSequentialSolver {
            route_service,
            candidate_chooser,
        };

        Self {
            grasp_solver: GraspSolver::new(
                parameters.max_improvement_times,
                Box::new(first_stage_solver),
                Box::new(TwoOptSearcher::new(distance_service)),
            ),
        }
    }
}
