use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::route::route_service::RouteService,
    solvers::genetic::genetic_solver::GeneticSolver,
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

    let route_service = RouteService::new(vehicles, &distances, stops.clone());

    let mut genetic_solver = GeneticSolver::new(
        stops,
        &distances,
        parameters.population_size,
        parameters.elite_size,
        parameters.mutation_rate,
        parameters.max_crossover_tries,
        parameters.max_generations,
        0.2,
        route_service,
        &mut rng,
    );

    genetic_solver.solve();

    genetic_solver
        .solution
        .routes
        .into_iter()
        .enumerate()
        .for_each(|(index, (vehicle_id, solution))| {
            let route = &mut *result.offset(index.try_into().unwrap());

            route.vehicle_id = vehicle_id;
            route.total_distance = solution.total_distance();
            let stop_ids: Vec<u32> = solution.stops.iter().map(|stop| stop.id).collect();

            route.number_of_stops = stop_ids.len();

            copy_result(stop_ids, route.stop_ids)
        });
}
