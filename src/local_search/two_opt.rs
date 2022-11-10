use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceService,
};

use super::path::Path;

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

pub(crate) fn calculate_swap_cost<'a>(
    path1: &Path,
    path2: &Path,
    distance_service: &DistanceService,
) -> f64 {
    let swapped_path_1 = Path::new(
        *path1.get_prev(),
        *path2.get_current(),
        *path1.get_next(),
        distance_service,
    )
    .unwrap();

    let swapped_path_2 = Path::new(
        *path2.get_prev(),
        *path1.get_current(),
        *path2.get_next(),
        distance_service,
    )
    .unwrap();

    swapped_path_1.get_cost() + swapped_path_2.get_cost()
}

pub(crate) fn calculate_minimum_swap_cost<'a>(
    stops: &'a Vec<Stop>,
    distance_service: &'a DistanceService,
    current_path: &'a Path<'a>,
) -> Option<(usize, f64)> {
    stops
        .windows(3)
        .enumerate()
        .skip(current_path.get_current().get_index() + 1)
        .map(|(next_stop_index, next_stop_window)| {
            (
                next_stop_index,
                calculate_swap_cost(
                    current_path,
                    &Path::from_window(next_stop_window, next_stop_index, distance_service)
                        .unwrap(),
                    distance_service,
                ),
            )
        })
        .min_by(|(_, cost1), (_, cost2)| cost1.partial_cmp(cost2).unwrap())
}

pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<bool> {
    for (prev_path_index, current_path_window) in route.get_stops().windows(3).enumerate() {
        let current_path =
            Path::from_window(current_path_window, prev_path_index, distance_service)?;

        let (min_swap_cost_index, swap_cost) =
            calculate_minimum_swap_cost(route.get_stops(), distance_service, &current_path)?;

        if swap_cost > current_path.get_cost() {
            return Some(true);
        }
    }

    return Some(false);
}
