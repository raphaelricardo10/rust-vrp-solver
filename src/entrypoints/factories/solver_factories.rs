use rand::Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    entrypoints::structures::{
        arg_sizes::ArgSizes, distance_matrix::FFIDistanceMatrixEntry,
        parameters::FFIGeneticSolverParameters, route::FFIRoute,
    },
    services::route::route_service::RouteService,
    solvers::{
        genetic::{
            crossover::crossover_operator::CrossoverOperator,
            genetic_solver::{GeneticSolver, GeneticSolverParameters},
            population::Population,
        },
        solution::Solution,
    },
};

use super::raw_factories::{copy_result, distance_matrix_factory, vector_factory};

pub(crate) unsafe fn genetic_solver_factory<'a, R>(
    vehicles_ptr: *mut Vehicle,
    stops_ptr: *mut Stop,
    distances_ptr: *mut FFIDistanceMatrixEntry,
    arg_sizes: ArgSizes,
    crossover_op: &'a dyn CrossoverOperator<R>,
    parameters: FFIGeneticSolverParameters,
    rng: &'a mut R,
) -> GeneticSolver<'a, R>
where
    R: Rng + ?Sized,
{
    let vehicles = vector_factory(vehicles_ptr, arg_sizes.vehicles);
    let stops = vector_factory(stops_ptr, arg_sizes.stops);

    let distances = distance_matrix_factory(distances_ptr, arg_sizes.distances);
    let mut route_service = RouteService::new(vehicles, &distances, stops.clone());
    let population = Population::from((parameters.population_size, &mut *rng, &mut route_service));

    let parameters = GeneticSolverParameters {
        elite_size: parameters.elite_size,
        mutation_rate: parameters.mutation_rate,
        max_generations: parameters.max_generations,
        local_search_rate: parameters.local_search_rate,
    };

    GeneticSolver::new(stops, &distances, population, parameters, crossover_op, rng)
}

pub(crate) unsafe fn copy_solution_to_abi(solution: Solution, result_ptr: *mut FFIRoute) {
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
