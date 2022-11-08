use std::collections::HashMap;

use crate::domain::stop::Stop;

use super::distance_matrix::DistanceMatrixEntry;

pub type DistancesMatrixKey = (u32, u32);
pub type DistanceMatrixInput = HashMap<DistancesMatrixKey, f64>;
pub(crate) type DistanceMatrix = HashMap<DistancesMatrixKey, DistanceMatrixEntry>;

pub type StopsMap = HashMap<u32, Stop>;

pub struct DistanceService {
    distances: DistanceMatrix,
}

impl<'a> DistanceService {
    pub fn new(stops: Vec<Stop>, distances: DistanceMatrixInput) -> DistanceService {
        DistanceService {
            distances: Self::map_distances(stops, distances),
        }
    }

    fn map_distances(stops: Vec<Stop>, distances: DistanceMatrixInput) -> DistanceMatrix {
        let stops_map: StopsMap = stops.iter().map(|stop| (stop.get_id(), *stop)).collect();

        distances
            .iter()
            .map(|x| {
                (
                    (x.0 .0, x.0 .1),
                    DistanceMatrixEntry::new(*stops_map.get(&x.0 .1).unwrap(), *x.1),
                )
            })
            .collect()
    }

    pub fn get_distance(&self, from: &Stop, to: &Stop) -> Option<f64> {
        Some(
            self.distances
                .get(&(from.get_id(), to.get_id()))?
                .get_distance(),
        )
    }

    pub(crate) fn get_distances_from(
        &'a self,
        stop: &'a Stop,
    ) -> impl Iterator<Item = &DistanceMatrixEntry> {
        self.distances
            .iter()
            .filter(|x| x.0 .0 == stop.get_id())
            .map(|x| x.1)
    }

    pub fn get_nearest_stop(
        &'a self,
        stop: &'a Stop,
        filter: impl Fn(&Stop) -> bool,
    ) -> Option<&Stop> {
        self.get_distances_from(stop)
            .filter(|entry| filter(entry.get_destination_stop()))
            .min_by(|stop1, stop2| stop1.partial_cmp(stop2).unwrap())
            .map(|x| x.get_destination_stop())
    }

    pub fn get_k_nearest_stops(
        &'a self,
        stop: &'a Stop,
        k: usize,
        filter: impl Fn(&Stop) -> bool,
    ) -> Vec<&Stop> {
        let mut stops = self
            .get_distances_from(stop)
            .filter(|entry| filter(entry.get_destination_stop()))
            .collect::<Vec<&DistanceMatrixEntry>>();

        stops.sort_by(|stop1, stop2| stop1.partial_cmp(stop2).unwrap());

        stops[0..k]
            .iter()
            .map(|x| x.get_destination_stop())
            .collect()
    }
}
