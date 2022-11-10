use crate::{
    domain::{route::Route, vehicle::Vehicle},
    local_search::{two_opt::{self, calculate_stop_insertion_cost, calculate_minimum_swap_cost, calculate_swap_cost}, path::Path},
    services::distance::distance_service::DistanceService,
};
use rstest::rstest;

use crate::domain::stop::Stop;

use super::fixtures::{distance_service, stops_with_crossings};

#[rstest]
fn can_calculate_insertion_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let insertion_cost =
        calculate_stop_insertion_cost(&stops_with_crossings, &distance_service, &1);

    assert_eq!(insertion_cost, 6.0);
}

#[rstest]
fn can_calculate_path_swap_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let path1 = Path::from_stop_index(&stops_with_crossings, 1, &distance_service).unwrap();
    let path2 = Path::from_stop_index(&stops_with_crossings, 3, &distance_service).unwrap();

    let swap_cost = calculate_swap_cost(&path1, &path2, &distance_service);

    assert_eq!(swap_cost - (path1.get_cost() + path2.get_cost()), 0.0);
}

#[rstest]
fn can_get_the_minimum_swap_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let stop_index = 1;
    let path = Path::from_stop_index(&stops_with_crossings, stop_index, &distance_service).unwrap();

    let swap_cost = calculate_minimum_swap_cost(&stops_with_crossings, &distance_service, &path).unwrap();
        
    assert_eq!(swap_cost.1, 12.0);
}

#[rstest]
fn can_optimize_route(distance_service: DistanceService, stops_with_crossings: Vec<Stop>) {
    let vehicle = Vehicle::new(0, 100);

    let mut route = Route::new(vehicle);

    route.add_stop(stops_with_crossings[0], 0.0).unwrap();
    route
        .add_stop(
            stops_with_crossings[1],
            distance_service
                .get_distance(&stops_with_crossings[0], &stops_with_crossings[1])
                .unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[2],
            distance_service
                .get_distance(&stops_with_crossings[1], &stops_with_crossings[2])
                .unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[3],
            distance_service
                .get_distance(&stops_with_crossings[2], &stops_with_crossings[3])
                .unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[4],
            distance_service
                .get_distance(&stops_with_crossings[3], &stops_with_crossings[4])
                .unwrap(),
        )
        .unwrap();

    let result = two_opt::search(&mut route, &distance_service).unwrap();

    assert!(result);
}
