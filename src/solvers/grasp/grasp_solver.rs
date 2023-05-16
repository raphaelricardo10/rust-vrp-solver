use crate::{
    local_search::local_searcher::LocalSearcher,
    solvers::{solution::Solution, solver::Solver},
};

pub trait GraspSolverCallbacks {
    fn before_iteration(&mut self) {}
    fn after_iteration(&mut self) {}
}

pub trait SolverWithGraspCallbacks<S: Solution>: Solver<S> + GraspSolverCallbacks {}

pub struct GraspSolver<S: Solution + Default> {
    max_improvement_times: u8,
    first_stage_solver: Box<dyn SolverWithGraspCallbacks<S>>,
    local_searcher: Box<dyn LocalSearcher<S> + 'static>,
}

impl<S> GraspSolver<S>
where
    S: Solution + Default,
{
    pub fn new(
        max_improvement_times: u8,
        first_stage_solver: Box<dyn SolverWithGraspCallbacks<S>>,
        local_searcher: Box<dyn LocalSearcher<S> + 'static>,
    ) -> Self {
        Self {
            local_searcher,
            max_improvement_times,
            first_stage_solver,
        }
    }
}

impl<S> Solver<S> for GraspSolver<S>
where
    S: Solution + Default,
{
    fn solve(&mut self) -> S {
        let mut best_solution = Default::default();
        let mut times_without_improvement: u8 = 0;

        while times_without_improvement < self.max_improvement_times {
            self.first_stage_solver.before_iteration();

            let mut solution = self.first_stage_solver.solve();
            self.local_searcher.run(&mut solution);

            if solution.is_better_than(&best_solution) {
                best_solution = solution;
                times_without_improvement = 0;
            } else {
                times_without_improvement += 1;
            }

            self.first_stage_solver.after_iteration();
        }

        best_solution
    }
}
