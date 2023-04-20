use rstest::rstest;

use crate::stop_swapper::StopSwapper;

use crate::tests::fixtures::stop_swapper_fixture::{
    neighborhood_factory, stop_swapper, NeighborhoodFactory,
};

#[rstest]
fn can_calculate_neighborhood_swap_cost(
    stop_swapper: StopSwapper,
    neighborhood_factory: NeighborhoodFactory,
) {
    let neighborhood1 = neighborhood_factory.make_neighborhood(1);
    let neighborhood2 = neighborhood_factory.make_neighborhood(3);

    let swap_cost = stop_swapper.calculate_swap_cost(&neighborhood1, &neighborhood2);

    assert_eq!(swap_cost, -4.0);
}

#[rstest]
fn can_get_the_minimum_swap_cost(
    stop_swapper: StopSwapper,
    neighborhood_factory: NeighborhoodFactory,
) {
    let neighborhood = neighborhood_factory.make_neighborhood(1);

    let swap_cost = stop_swapper.get_minimum_swap_cost(&neighborhood, &neighborhood_factory.stops);

    assert_eq!(swap_cost.1, -5.0);
}

#[rstest]
fn can_calculate_neighborhood_swap_cost_of_consecutive_swaps(
    stop_swapper: StopSwapper,
    neighborhood_factory: NeighborhoodFactory,
) {
    let neighborhood1 = neighborhood_factory.make_neighborhood(1);
    let neighborhood2 = neighborhood_factory.make_neighborhood(2);

    let swap_cost1 = stop_swapper.calculate_swap_cost(&neighborhood1, &neighborhood2);
    let swap_cost2 = stop_swapper.calculate_swap_cost(&neighborhood2, &neighborhood1);

    assert_eq!(swap_cost1, -4.5);
    assert_eq!(swap_cost2, -4.5);
}
