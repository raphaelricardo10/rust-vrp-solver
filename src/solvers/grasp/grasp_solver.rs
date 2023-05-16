use crate::{
    local_search::local_search::LocalSearch,
    solvers::{solution::Solution, solver::Solver},
};

pub trait GraspSolverCallbacks {
    fn before_iteration(&mut self) {}
    fn after_iteration(&mut self) {}
}

pub trait SolverWithGraspCallbacks<S: Solution>: Solver<S> + GraspSolverCallbacks {}

pub struct GraspSolver<S: Solution + Default + Clone> {
    max_improvement_times: u8,
    times_without_improvement: u8,
    best_solution: S,
    first_stage_solver: Box<dyn SolverWithGraspCallbacks<S>>,
    local_searcher: Box<dyn LocalSearch<S> + 'static>,
}

impl<S> GraspSolver<S>
where
    S: Solution + Default + Clone,
{
    pub fn new(
        max_improvement_times: u8,
        first_stage_solver: Box<dyn SolverWithGraspCallbacks<S>>,
        local_searcher: Box<dyn LocalSearch<S> + 'static>,
    ) -> Self {
        Self {
            local_searcher,
            max_improvement_times,
            first_stage_solver,
            best_solution: Default::default(),
            times_without_improvement: Default::default(),
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.times_without_improvement >= self.max_improvement_times
    }
}

impl<S> Solver<S> for GraspSolver<S>
where
    S: Solution + Default + Clone,
{
    fn solve(&mut self) -> S {
        while !self.stop_condition_met() {
            self.first_stage_solver.before_iteration();

            let mut solution = self.first_stage_solver.solve();
            self.local_searcher.run(&mut solution);

            if solution.is_better_than(&self.best_solution) {
                self.best_solution = solution;
                self.times_without_improvement = 0;
            } else {
                self.times_without_improvement += 1;
            }

            self.first_stage_solver.after_iteration();
        }

        self.best_solution.clone()
    }
}
