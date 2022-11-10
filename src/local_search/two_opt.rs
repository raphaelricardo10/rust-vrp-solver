use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceService,
};

use super::path::Path;

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

pub(crate) fn get_minimum_swap_cost<'a>(
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

fn should_swap_stops(path1: &Path, path2: &Path, swap_cost: &f64) -> bool{
    *swap_cost < path1.get_cost() + path2.get_cost()
}

pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<bool> {
    for (base_index, window) in route.get_stops().windows(3).enumerate() {
        let path1 =
            Path::from_window(window, base_index, distance_service)?;

        let (swap_candidate_index, swap_cost) =
            get_minimum_swap_cost(route.get_stops(), distance_service, &path1)?;

        let path2 = Path::from_stop_index(route.get_stops(), swap_candidate_index, distance_service)?;

        if should_swap_stops(&path1, &path2, &swap_cost){
            return Some(true);
        }
    }

    return Some(false);
}
