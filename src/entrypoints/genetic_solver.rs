use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::genetic::genetic_solver::GeneticSolver,
};

use super::parameters::GeneticAlgorithmParameters;

#[no_mangle]
pub extern "C" fn genetic_solver(
    vehicles_ptr: *mut Vehicle,
    stops_ptr: *mut Stop,
    parameters: GeneticAlgorithmParameters,
    distances_ptr: *mut f64,
    result: *mut u32,
) {
    let mut rng = thread_rng();

    todo!();

    // let vehicles = make_vector(vehicles_ptr, parameters.number_of_routes);
    // let stops = make_vector(stops_ptr, parameters.number_of_stops);

    // let distances = &make_distances();

    // let route_service = RouteService::new(vehicles, distances, stops.clone());

    // let mut genetic_solver = GeneticSolver::new(
    //     stops,
    //     distances,
    //     parameters.population_size,
    //     parameters.elite_size,
    //     parameters.mutation_rate,
    //     parameters.max_crossover_tries,
    //     parameters.max_generations,
    //     route_service,
    //     &mut rng,
    // );

    // genetic_solver.solve();
}
