use crate::{
    domain::{route::Route, vehicle::Vehicle},
    local_search::two_opt::{self, calculate_stop_insertion_cost, calculate_stop_swap_cost},
    services::distance::distance_service::DistanceService,
};
use rstest::rstest;

use crate::domain::stop::Stop;

use super::fixtures::{distance_service, stops_with_crossings};

#[rstest]
fn can_calculate_insertion_cost(distance_service: DistanceService, stops_with_crossings: Vec<Stop>) {
    let insertion_cost = calculate_stop_insertion_cost(&stops_with_crossings, &distance_service, &1);

    assert_eq!(insertion_cost, 6.0);
}

#[rstest]
fn can_calculate_path_swap_cost(distance_service: DistanceService, stops_with_crossings: Vec<Stop>){
    let insertion_cost_1 = calculate_stop_insertion_cost(&stops_with_crossings, &distance_service, &1);
    let insertion_cost_2 = calculate_stop_insertion_cost(&stops_with_crossings, &distance_service, &3);
    let swap_cost = calculate_stop_swap_cost(&stops_with_crossings, &distance_service, &1, &3).unwrap();

    assert_eq!(swap_cost - (insertion_cost_1 + insertion_cost_2), 0.0);
}

#[rstest]
fn can_optimize_route(distance_service: DistanceService, stops_with_crossings: Vec<Stop>) {
    let vehicle = Vehicle::new(0, 100);

    let mut route = Route::new(vehicle);

    route.add_stop(stops_with_crossings[0], 0.0).unwrap();
    route
        .add_stop(
            stops_with_crossings[1],
            distance_service.get_distance(&stops_with_crossings[0], &stops_with_crossings[1]).unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[2],
            distance_service.get_distance(&stops_with_crossings[1], &stops_with_crossings[2]).unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[3],
            distance_service.get_distance(&stops_with_crossings[2], &stops_with_crossings[3]).unwrap(),
        )
        .unwrap();
    route
        .add_stop(
            stops_with_crossings[4],
            distance_service.get_distance(&stops_with_crossings[3], &stops_with_crossings[4]).unwrap(),
        )
        .unwrap();

    let result = two_opt::search(&mut route, &distance_service);

    assert!(result);
}
