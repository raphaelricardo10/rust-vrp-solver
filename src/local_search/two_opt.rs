use std::collections::BTreeMap;

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
    path: &'a Path<'a>,
) -> Option<(usize, f64)> {
    stops
        .windows(3)
        .enumerate()
        .skip(path.get_next().get_index())
        .map(|(base_index, next_stop_window)| {
            (
                base_index + 1,
                calculate_swap_cost(
                    path,
                    &Path::from_window(next_stop_window, base_index, distance_service)
                        .unwrap(),
                    distance_service,
                ),
            )
        })
        .min_by(|(_, cost1), (_, cost2)| cost1.partial_cmp(cost2).unwrap())
}

fn should_swap_stops(path1: &Path, path2: &Path, swap_cost: &f64) -> bool {
    *swap_cost < path1.get_cost() + path2.get_cost()
}

fn find_improvements(
    stops: &Vec<Stop>,
    distance_service: &DistanceService,
    path: &Path,
) -> Option<usize> {
    let (swap_candidate_index, swap_cost) = get_minimum_swap_cost(stops, distance_service, &path)?;

    let swap_candidate = Path::from_stop_index(stops, swap_candidate_index, distance_service)?;

    if should_swap_stops(&path, &swap_candidate, &swap_cost) {
        return Some(swap_candidate.get_prev().get_index())
    }

    None
}

fn map_paths<'a>(
    stops: &'a Vec<Stop>,
    distance_service: &'a DistanceService,
) -> BTreeMap<usize, Path<'a>> {
    stops
        .windows(3)
        .enumerate()
        .map(|(base_index, window)| {
            (
                base_index,
                Path::from_window(window, base_index, distance_service).unwrap(),
            )
        })
        .collect()
}

pub(crate) fn execute_swap(path_map: &mut BTreeMap<usize, Path>, index1: usize, index2: usize, distance_service: &DistanceService) {
    let mut path1 = *path_map.get(&index1).unwrap();
    let mut path2 = *path_map.get(&index2).unwrap();
    let aux = *path1.get_current();

    path1.set_current(*path2.get_current(), distance_service);
    path2.set_current(aux, distance_service);

    path_map.insert(path1.get_prev().get_index(), path1);
    path_map.insert(path2.get_prev().get_index(), path2);
}

pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<bool> {
    let mut paths = map_paths(route.get_stops(), distance_service);

    for stop_index in 0..paths.len() {
        let path = paths.get(&stop_index)?;
        
        let swap_candidate_index = match find_improvements(route.get_stops(), distance_service, &path) {
            Some(candidate_index) => candidate_index,
            None => continue,
        };
        
        let base_index = path.get_prev().get_index();
        execute_swap(&mut paths, base_index, swap_candidate_index, distance_service)
    }

    return Some(false);
}
