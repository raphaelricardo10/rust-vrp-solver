use crate::stop_swapper::neighborhood::Neighborhood;
use crate::{
    local_search::two_opt::TwoOptSearcher, services::distance::distance_service::DistanceService,
};
use rstest::rstest;

use crate::domain::stop::Stop;

use crate::tests::fixtures::routes_fixture::{route_factory, RouteFactory};
use crate::tests::fixtures::services_fixture::distance_service;
use crate::tests::fixtures::stops_fixture::stops_with_crossings;
use crate::tests::fixtures::two_opt::two_opt;

#[rstest]
fn can_calculate_insertion_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let neighborhood =
        Neighborhood::from_stop_index(&stops_with_crossings, 1, &distance_service).unwrap();

    assert_eq!(neighborhood.cost, 8.0);
}

#[rstest]
fn can_optimize_route(
    two_opt: TwoOptSearcher,
    route_factory: RouteFactory,
    stops_with_crossings: Vec<Stop>,
) {
    let mut route = route_factory(stops_with_crossings);

    two_opt.run(&mut route).unwrap();

    assert_eq!(route.stops.get(0).unwrap().id, 0);
    assert_eq!(route.stops.get(1).unwrap().id, 2);
    assert_eq!(route.stops.get(2).unwrap().id, 3);
    assert_eq!(route.stops.get(3).unwrap().id, 1);
    assert_eq!(route.stops.get(4).unwrap().id, 4);
    assert_eq!(route.stops.get(5).unwrap().id, 0);

    assert_eq!(route.total_distance(), 11.5);
}
