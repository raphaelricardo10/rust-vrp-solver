use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::route::route_service::RouteService,
    solvers::{genetic::{
        crossover::order_crossover::OrderCrossover,
        genetic_solver::{GeneticSolver, GeneticSolverParameters},
        population::Population,
    }, solver::Solver},
};

use super::abi::{
    arg_sizes::ArgSizes, distance_matrix::ABIDistanceMatrixEntry,
    parameters::GeneticAlgorithmParameters, route::ABIRoute,
};

use super::factories::{copy_result, distance_matrix_factory, vector_factory};

/// # Safety
///
/// Make sure that all the values in arg_sizes are consistent
/// with the input pointers.
#[no_mangle]
pub unsafe extern "C" fn genetic_solver(
    vehicles_ptr: *mut Vehicle,
    stops_ptr: *mut Stop,
    distances_ptr: *mut ABIDistanceMatrixEntry,
    arg_sizes: ArgSizes,
    parameters: GeneticAlgorithmParameters,
    result: *mut ABIRoute,
) {
    let mut rng = thread_rng();

    let vehicles = vector_factory(vehicles_ptr, arg_sizes.vehicles);
    let stops = vector_factory(stops_ptr, arg_sizes.stops);

    let distances = distance_matrix_factory(distances_ptr, arg_sizes.distances);
    let mut route_service = RouteService::new(vehicles, &distances, stops.clone());
    let population = Population::from((parameters.population_size, &mut rng, &mut route_service));

    let crossover_op = OrderCrossover::new(parameters.max_crossover_tries);

    let parameters = GeneticSolverParameters {
        elite_size: parameters.elite_size,
        mutation_rate: parameters.mutation_rate,
        max_generations: parameters.max_generations,
        local_search_rate: parameters.local_search_rate,
    };

    let mut genetic_solver = GeneticSolver::new(
        stops,
        &distances,
        population,
        parameters,
        &crossover_op,
        &mut rng,
    );

    genetic_solver.solve();

    genetic_solver
        .solution
        .routes
        .into_iter()
        .enumerate()
        .for_each(|(index, (vehicle_id, solution))| {
            let route = &mut *result.offset(
                index
                    .try_into()
                    .expect("the index should fit in memory address size"),
            );

            route.vehicle_id = vehicle_id;
            route.total_distance = solution.total_distance();
            let stop_ids: Vec<u32> = solution.stops.iter().map(|stop| stop.id).collect();

            route.number_of_stops = stop_ids.len();

            copy_result(stop_ids, route.stop_ids)
        });
}
