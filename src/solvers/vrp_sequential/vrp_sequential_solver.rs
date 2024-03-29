use crate::{
    services::route::route_service::RouteService,
    solvers::{
        sequential::sequential_solver::{
            CandidateChooser, SequentialSolver, SequentialSolverParameters, SolutionGetter,
        },
        solver::SolverCallbacks,
        vrp_solution::VrpSolution,
    },
};

pub struct VrpSequentialSolver {
    pub route_service: RouteService,
    pub candidate_chooser: Box<dyn CandidateChooser<Self>>,
}

impl SolverCallbacks for VrpSequentialSolver {
    fn before_solving(&mut self) {
        self.route_service.assign_starting_points();
    }

    fn after_solving(&mut self) {
        self.route_service.assign_stop_points();
    }

    fn on_exit(&mut self) {
        self.route_service.reset();
    }
}

impl SequentialSolverParameters for VrpSequentialSolver {
    type SequenceId = u32;
    type CandidateId = u32;
    type Cost = f32;
}

impl SolutionGetter<VrpSolution> for VrpSequentialSolver {
    fn get_solution(&self) -> VrpSolution {
        VrpSolution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        )
    }
}

impl SequentialSolver<Self> for VrpSequentialSolver {
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

    fn get_candidate_chooser(&self) -> &dyn CandidateChooser<Self> {
        self.candidate_chooser.as_ref()
    }
}
