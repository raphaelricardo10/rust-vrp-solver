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

    pub(crate) fn calculate_swap_cost(&self, path1: &Path, path2: &Path) -> f64 {
        let swapped_path_1 = Path::new(
            path1.prev,
            path2.current,
            path1.next,
            &self.distance_service,
        )
        .unwrap();

        let swapped_path_2 = Path::new(
            path2.prev,
            path1.current,
            path2.next,
            &self.distance_service,
        )
        .unwrap();

        swapped_path_1.cost + swapped_path_2.cost
    }

    pub(crate) fn get_minimum_swap_cost(
        &self,
        path: &Path,
        stops: &Vec<Stop>,
    ) -> Option<(usize, f64)> {
        stops[..stops.len() - 1]
            .iter()
            .enumerate()
            .skip(path.next.index + 1)
            .map(|(stop_index, _)| {
                (
                    stop_index,
                    self.calculate_swap_cost(
                        path,
                        &Path::from_stop_index(stops, stop_index, &self.distance_service).unwrap(),
                    ),
                )
            })
            .min_by(|(_, cost1), (_, cost2)| cost1.partial_cmp(cost2).unwrap())
    }

    fn should_swap_stops(path1: &Path, path2: &Path, swap_cost: &f64) -> bool {
        *swap_cost < path1.cost + path2.cost
    }

    fn find_improvements(&self, stops: &Vec<Stop>, path: &Path) -> Option<usize> {
        let (swap_candidate_index, swap_cost) = self.get_minimum_swap_cost(path, stops)?;

        let swap_candidate =
            Path::from_stop_index(stops, swap_candidate_index, &self.distance_service)?;

        if Self::should_swap_stops(&path, &swap_candidate, &swap_cost) {
            return Some(swap_candidate.current.index);
        }

        None
    }

    pub fn search(&self, route: &mut Route) -> Option<()> {
        for stop_index in 1..route.stops.len() - 1 {
            let path = Path::from_stop_index(&route.stops, stop_index, &self.distance_service)?;

            let swap_candidate_index = match self.find_improvements(&route.stops, &path) {
                Some(candidate_index) => candidate_index,
                None => continue,
            };

            let base_index = path.current.index;
            route.stops.swap(base_index, swap_candidate_index);
        }

        Some(())
    }
}
