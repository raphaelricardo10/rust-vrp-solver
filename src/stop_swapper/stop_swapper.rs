use crate::{
    domain::stop::Stop,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::path::Path;

pub(crate) struct StopSwapper {
    pub(crate) distance_service: DistanceService,
}

impl StopSwapper {
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

        (swapped_path_1.cost + swapped_path_2.cost) - (path1.cost + path2.cost)
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
}