use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    solvers::{genetic::crossover::order_crossover::OrderCrossover, solver::Solver},
};

use super::{
    abi::{
        arg_sizes::ArgSizes, distance_matrix::ABIDistanceMatrixEntry,
        parameters::GeneticAlgorithmParameters, route::ABIRoute,
    },
    factories::solver_factories::{copy_solution_to_abi, genetic_solver_factory},
};

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

    let crossover_op = OrderCrossover::new(parameters.max_crossover_tries);
    let mut genetic_solver = genetic_solver_factory(
        vehicles_ptr,
        stops_ptr,
        distances_ptr,
        arg_sizes,
        &crossover_op,
        parameters,
        &mut rng,
    );

    genetic_solver.solve();

    copy_solution_to_abi(genetic_solver.solution, result);
}
