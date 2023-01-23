use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::route::route_service::RouteService,
    solvers::genetic::genetic_solver::GeneticSolver,
};

use super::abi::{
    arg_sizes::ArgSizes, distance_matrix::ABIDistanceMatrixEntry,
    parameters::GeneticAlgorithmParameters,
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
    result: *mut u32,
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
        route_service,
        &mut rng,
    );

    genetic_solver.solve();

    copy_result(
        genetic_solver.solution.result.get(&0).unwrap().to_vec(),
        result,
    );
}
