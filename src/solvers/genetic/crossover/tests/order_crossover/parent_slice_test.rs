use crate::solvers::genetic::individual::Individual;
use crate::solvers::genetic::tests::fixtures::{individual_factory, IndividualFactory};
use crate::tests::fixtures::services_fixture::distance_service;

use rstest::rstest;

use crate::services::distance::distance_service::DistanceService;

#[rstest]
fn test_slice_cost_is_correct(
    distance_service: DistanceService,
    mut individual_factory: IndividualFactory,
) {
    let individual = individual_factory(1);
    let route = &individual.chromosomes[0];
    let slice_cost = Individual::calculate_slice_cost(&route.stops, &distance_service);

    assert_eq!(slice_cost, route.total_distance());
}
