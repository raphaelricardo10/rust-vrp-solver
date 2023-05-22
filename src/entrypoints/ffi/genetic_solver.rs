use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    solvers::{
        genetic::crossover::order_crossover::OrderCrossover,
        grasp::vrp_grasp_solver::GraspSolverParameters, solver::Solver,
    },
};

use super::{
    factories::solver_factories::{
        copy_solution_to_abi, grasp_solver_factory, two_stage_genetic_solver_factory,
    },
    structures::{
        arg_sizes::ArgSizes, distance_matrix::FFIDistanceMatrixEntry,
        parameters::FFIGeneticSolverParameters, route::FFIRoute,
    },
};

/// # Safety
///
/// Make sure that all the values in arg_sizes are consistent
/// with the input pointers.
#[no_mangle]
pub unsafe extern "C" fn grasp_genetic_solver(
    vehicles_ptr: *mut Vehicle,
    stops_ptr: *mut Stop,
    distances_ptr: *mut FFIDistanceMatrixEntry,
    arg_sizes: ArgSizes,
    grasp_solver_parameters: GraspSolverParameters,
    genetic_solver_parameters: FFIGeneticSolverParameters,
    result_ptr: *mut FFIRoute,
) {
    let mut grasp_solver = grasp_solver_factory(
        vehicles_ptr,
        stops_ptr,
        distances_ptr,
        arg_sizes,
        grasp_solver_parameters,
        Box::new(ChaCha20Rng::from_entropy()),
    );

    let mut crossover_op = OrderCrossover::new(genetic_solver_parameters.max_crossover_tries);

    let mut genetic_solver = two_stage_genetic_solver_factory(
        stops_ptr,
        distances_ptr,
        arg_sizes,
        &mut grasp_solver,
        &mut crossover_op,
        genetic_solver_parameters,
        Box::new(ChaCha20Rng::from_entropy()),
    );

    let solution = genetic_solver.solve();

    copy_solution_to_abi(solution, result_ptr);
}
