use rstest::rstest;

use crate::{
    domain::stop::Stop,
    services::distance::distance_service::DistanceService,
    stop_swapper::{path::Path, StopSwapper},
};

use super::fixtures::{distance_service, stop_swapper, stops_with_crossings};

#[rstest]
fn can_calculate_path_swap_cost(
    stop_swapper: StopSwapper,
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let path1 = Path::from_stop_index(&stops_with_crossings, 1, &distance_service).unwrap();
    let path2 = Path::from_stop_index(&stops_with_crossings, 3, &distance_service).unwrap();

    let swap_cost = stop_swapper.calculate_swap_cost(&path1, &path2);

    assert_eq!(swap_cost, -4.0);
}

#[rstest]
fn can_get_the_minimum_swap_cost(
    stop_swapper: StopSwapper,
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let stop_index = 1;
    let path = Path::from_stop_index(&stops_with_crossings, stop_index, &distance_service).unwrap();

    let swap_cost = stop_swapper
        .get_minimum_swap_cost(&path, &stops_with_crossings)
        .unwrap();

    assert_eq!(swap_cost.1, -5.0);
}
