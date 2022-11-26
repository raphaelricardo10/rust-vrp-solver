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

    fn are_paths_consecutive(path1: &Path, path2: &Path) -> bool {
        if path1.next.index == path2.current.index {
            return true;
        }

        if path1.prev.index == path2.current.index {
            return true;
        }

        false
    }

    fn swap_non_consecutive_paths<'a>(
        path1: &'a Path<'a>,
        path2: &'a Path<'a>,
        distance_service: &'a DistanceService,
    ) -> (Path<'a>, Path<'a>) {
        let swapped_path_1 =
            Path::new(path1.prev, path2.current, path1.next, distance_service).unwrap();

        let swapped_path_2 =
            Path::new(path2.prev, path1.current, path2.next, distance_service).unwrap();

        (swapped_path_1, swapped_path_2)
    }

    fn swap_consecutive_paths<'a>(
        mut path1: &'a Path<'a>,
        mut path2: &'a Path<'a>,
        distance_service: &'a DistanceService,
    ) -> (Path<'a>, Path<'a>) {
        if path1.prev.stop.id == path2.current.stop.id {
            std::mem::swap(&mut path1, &mut path2);
        }

        let swapped_path_1 =
            Path::new(path1.prev, path2.current, path1.current, distance_service).unwrap();

        let swapped_path_2 =
            Path::new(path2.current, path1.current, path2.next, distance_service).unwrap();

        (swapped_path_1, swapped_path_2)
    }

    pub(crate) fn calculate_swap_cost(&self, path1: &Path, path2: &Path) -> f64 {
        let (swapped_path1, swapped_path2);

        if Self::are_paths_consecutive(path1, path2) {
            (swapped_path1, swapped_path2) =
                Self::swap_consecutive_paths(path1, path2, &self.distance_service);
        } else {
            (swapped_path1, swapped_path2) =
                Self::swap_non_consecutive_paths(path1, path2, &self.distance_service);
        }

        (swapped_path1.cost + swapped_path2.cost) - (path1.cost + path2.cost)
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
