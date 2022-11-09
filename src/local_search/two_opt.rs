use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceService,
};

pub fn calculate_stop_insertion_cost(
    stops: &Vec<Stop>,
    distance_service: &DistanceService,
    path_index: &usize,
) -> f64 {
    let lower_limit = path_index - 1;
    let upper_limit = path_index + 1;

    stops[lower_limit..=upper_limit]
        .windows(2)
        .map(|w| distance_service.get_distance(&w[0], &w[1]).unwrap())
        .sum()
}

pub fn calculate_stop_swap_cost(
    stops: &Vec<Stop>,
    distance_service: &DistanceService,
    path1_index: &usize,
    path2_index: &usize,
) -> Option<f64> {
    Some(
        distance_service.get_distance(&stops[path1_index - 1], &stops[*path2_index])?
            + distance_service.get_distance(&stops[*path2_index], &stops[path1_index + 1])?
            + distance_service.get_distance(&stops[path2_index - 1], &stops[*path1_index])?
            + distance_service.get_distance(&stops[*path1_index], &stops[path2_index + 1])?,
    )
}

pub fn calculate_minimum_swap_cost(
    stops: &Vec<Stop>,
    distance_service: &DistanceService,
    current_stop_index: &usize,
) -> Option<(usize, f64)> {
    stops
        .windows(2)
        .enumerate()
        .skip(current_stop_index + 2)
        .map(|(next_stop_index, _)| {
            (
                next_stop_index,
                calculate_stop_swap_cost(
                    stops,
                    distance_service,
                    current_stop_index,
                    &next_stop_index,
                )
                .unwrap(),
            )
        })
        .min_by(|path1, path2| path1.1.partial_cmp(&path2.1).unwrap())
}

pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<bool> {
    for (current_stop_index, current_path) in route.get_stops().windows(2).enumerate().skip(1) {
        let current_insertion_cost =
            calculate_stop_insertion_cost(route.get_stops(), distance_service, &current_stop_index);

        let (next_stop_index, min_swap_cost) =
            calculate_minimum_swap_cost(route.get_stops(), distance_service, &current_stop_index)?;

        if min_swap_cost > current_insertion_cost {
            return Some(true);
        }
    }

    return Some(false);
}
