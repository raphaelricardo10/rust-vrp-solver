use crate::solvers::{greedy::tests::solver::TestGreedySolver, solution::Solution, solver::Solver};
use rstest::rstest;

#[rstest]
fn test_greedy_solution_is_correct() {
    let candidates = &[0, 1, 2, 3];
    let sequences = &[0, 1];
    let costs = &[(0, 2), (1, 3), (2, 1), (3, 0)];

    let mut solver = TestGreedySolver::new(candidates, sequences, costs);

    let solution = solver.solve();

    let sequence_1 = solution.get_data().get(&0).unwrap();
    let sequence_2 = solution.get_data().get(&1).unwrap();

    assert_eq!(*sequence_1, vec![3, 0]);
    assert_eq!(*sequence_2, vec![2, 1]);
    assert_eq!(solution.get_cost(), 6);
}
