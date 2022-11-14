use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::path::Path;

pub struct TwoOptSearcher {
    distance_service: DistanceService,
}

impl TwoOptSearcher {
    pub fn new(stops: Vec<Stop>, distances: &DistanceMatrix) -> Self {
        Self {
            distance_service: DistanceService::new(stops, distances),
        }
    }

    pub(crate) fn calculate_swap_cost(
        path1: &Path,
        path2: &Path,
        distance_service: &DistanceService,
    ) -> f64 {
        let swapped_path_1 =
            Path::new(path1.prev, path2.current, path1.next, distance_service).unwrap();

        let swapped_path_2 =
            Path::new(path2.prev, path1.current, path2.next, distance_service).unwrap();

        swapped_path_1.cost + swapped_path_2.cost
    }

    pub(crate) fn get_minimum_swap_cost(
        path: &Path,
        stops: &Vec<Stop>,
        distance_service: &DistanceService,
    ) -> Option<(usize, f64)> {
        stops[..stops.len() - 1]
            .iter()
            .enumerate()
            .skip(path.next.index + 1)
            .map(|(stop_index, _)| {
                (
                    stop_index,
                    Self::calculate_swap_cost(
                        path,
                        &Path::from_stop_index(stops, stop_index, distance_service).unwrap(),
                        distance_service,
                    ),
                )
            })
            .min_by(|(_, cost1), (_, cost2)| cost1.partial_cmp(cost2).unwrap())
    }

    fn should_swap_stops(path1: &Path, path2: &Path, swap_cost: &f64) -> bool {
        *swap_cost < path1.cost + path2.cost
    }

    fn find_improvements(
        stops: &Vec<Stop>,
        distance_service: &DistanceService,
        path: &Path,
    ) -> Option<usize> {
        let (swap_candidate_index, swap_cost) =
            Self::get_minimum_swap_cost(path, stops, distance_service)?;

        let swap_candidate = Path::from_stop_index(stops, swap_candidate_index, distance_service)?;

        if Self::should_swap_stops(&path, &swap_candidate, &swap_cost) {
            return Some(swap_candidate.current.index);
        }

        None
    }

    pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<()> {
        for stop_index in 1..route.stops.len() - 1 {
            let path = Path::from_stop_index(&route.stops, stop_index, distance_service)?;

            let swap_candidate_index =
                match Self::find_improvements(&route.stops, distance_service, &path) {
                    Some(candidate_index) => candidate_index,
                    None => continue,
                };

            let base_index = path.current.index;
            route.stops.swap(base_index, swap_candidate_index);
        }

        Some(())
    }
}
