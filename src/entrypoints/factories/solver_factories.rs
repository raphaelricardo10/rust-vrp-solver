use rand::Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    entrypoints::structures::{
        arg_sizes::ArgSizes, distance_matrix::FFIDistanceMatrixEntry,
        parameters::FFIGeneticSolverParameters, route::FFIRoute,
    },
    solvers::{
        genetic::{
            crossover::crossover_operator::CrossoverOperator,
            genetic_solver::GeneticSolverParameters,
        },
        grasp::grasp_solver::{GraspSolver, GraspSolverParameters},
        solver::Solver,
        two_stage_genetic::two_stage_genetic_solver::{
            TwoStageGeneticSolver, TwoStageGeneticSolverParameters,
        },
        vrp_solution::VrpSolution,
    },
};

use super::raw_factories::{copy_result, distance_matrix_factory, vector_factory};

pub(crate) unsafe fn grasp_solver_factory<R>(
    vehicles_ptr: *mut Vehicle,
    stops_ptr: *mut Stop,
    distances_ptr: *mut FFIDistanceMatrixEntry,
    arg_sizes: ArgSizes,
    parameters: GraspSolverParameters,
    rng: Box<R>,
) -> GraspSolver<R>
where
    R: Rng + ?Sized,
{
    let stops = vector_factory(stops_ptr, arg_sizes.stops);
    let vehicles = vector_factory(vehicles_ptr, arg_sizes.vehicles);
    let distances = distance_matrix_factory(distances_ptr, arg_sizes.distances);

    GraspSolver::new(stops, vehicles, &distances, parameters, rng)
}

pub(crate) unsafe fn two_stage_genetic_solver_factory<'a, R>(
    stops_ptr: *mut Stop,
    distances_ptr: *mut FFIDistanceMatrixEntry,
    arg_sizes: ArgSizes,
    first_stage_solver: &'a mut dyn Solver<VrpSolution>,
    crossover_op: &'a mut dyn CrossoverOperator<R>,
    parameters: FFIGeneticSolverParameters,
    rng: Box<R>,
) -> TwoStageGeneticSolver<'a, R>
where
    R: Rng + ?Sized,
{
    let stops = vector_factory(stops_ptr, arg_sizes.stops);
    let distances = distance_matrix_factory(distances_ptr, arg_sizes.distances);

    let parameters = TwoStageGeneticSolverParameters {
        population_size: parameters.population_size,
        genetic_solver_parameters: GeneticSolverParameters {
            elite_size: parameters.elite_size,
            mutation_rate: parameters.mutation_rate,
            max_generations: parameters.max_generations,
            local_search_rate: parameters.local_search_rate,
        },
    };

    TwoStageGeneticSolver::new(
        stops,
        &distances,
        first_stage_solver,
        parameters,
        crossover_op,
        rng,
    )
}

pub(crate) unsafe fn copy_solution_to_abi(solution: VrpSolution, result_ptr: *mut FFIRoute) {
    solution
        .routes
        .into_iter()
        .enumerate()
        .for_each(|(index, (vehicle_id, solution))| {
            let route = &mut *result_ptr.offset(
                index
                    .try_into()
                    .expect("the index should fit in memory address size"),
            );

            route.vehicle_id = vehicle_id;
            route.total_distance = solution.total_distance();
            let stop_ids: Vec<u32> = solution.stops.iter().map(|stop| stop.id).collect();

            route.number_of_stops = stop_ids.len();

            copy_result(stop_ids, route.stop_ids);
        });
}
