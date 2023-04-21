use std::{cmp::min, collections::HashMap};

use crate::domain::stop::Stop;

use super::distance_matrix::DistanceMatrixEntry;

pub type DistancesMatrixKey = (u32, u32);
pub type DistanceMatrix = HashMap<DistancesMatrixKey, f64>;
pub(super) type MappedDistanceMatrix = HashMap<DistancesMatrixKey, DistanceMatrixEntry>;

pub type StopsMap = HashMap<u32, Stop>;

pub struct DistanceService {
    distances: MappedDistanceMatrix,
}

impl<'a> DistanceService {
    pub fn new(stops: Vec<Stop>, distances: &DistanceMatrix) -> DistanceService {
        DistanceService {
            distances: Self::map_distances(stops, distances),
        }
    }

    fn map_distances(stops: Vec<Stop>, distances: &DistanceMatrix) -> MappedDistanceMatrix {
        let stops_map: StopsMap = stops.iter().map(|stop| (stop.id, *stop)).collect();

        distances
            .iter()
            .map(|((source_stop_id, destination_stop_id), distance)| {
                (
                    (*source_stop_id, *destination_stop_id),
                    DistanceMatrixEntry::new(
                        *stops_map.get(destination_stop_id).unwrap_or_else(|| {
                            panic!("the stops should contain the stop {destination_stop_id}")
                        }),
                        *distance,
                    ),
                )
            })
            .collect()
    }

    pub fn get_distance(&self, from: &Stop, to: &Stop) -> f64 {
        self.distances
            .get(&(from.id, to.id))
            .unwrap_or_else(|| {
                panic!(
                    "the distance matrix should contain the entry [{0}, {1}]",
                    from.id, to.id
                )
            })
            .distance
    }

    pub(super) fn get_distances_from(
        &'a self,
        stop: &'a Stop,
    ) -> impl Iterator<Item = &DistanceMatrixEntry> {
        self.distances
            .iter()
            .filter(|x| x.0 .0 == stop.id)
            .map(|x| x.1)
    }

    pub fn get_nearest_stop(
        &'a self,
        stop: &'a Stop,
        filter: impl Fn(&Stop) -> bool,
    ) -> Option<&Stop> {
        self.get_distances_from(stop)
            .filter(|entry| filter(&entry.destination))
            .min_by(|stop1, stop2| {
                stop1.partial_cmp(stop2).unwrap_or_else(|| {
                    panic!(
                        "it should be possible to compare stop {0} and stop {1}, whose distances are respectively: {2} and {3}",
                        stop1.destination.id, stop2.destination.id, stop1.distance, stop2.distance
                    )
                })
            })
            .map(|x| &x.destination)
    }

    pub fn get_k_nearest_stops(
        &'a self,
        stop: &'a Stop,
        k: usize,
        filter: impl Fn(&Stop) -> bool,
    ) -> Vec<&Stop> {
        let mut stops = self
            .get_distances_from(stop)
            .filter(|entry| filter(&entry.destination))
            .collect::<Vec<&DistanceMatrixEntry>>();

        stops.sort_by(|stop1, stop2| {
            stop1.partial_cmp(stop2).unwrap_or_else(|| {
                panic!(
                    "it should be possible to compare stop {0} and stop {1}, whose distances are respectively: {2} and {3}",
                    stop1.destination.id, stop2.destination.id, stop1.distance, stop2.distance
                )
            })
        });

        let number_of_stops = min(stops.len(), k);

        stops[0..number_of_stops]
            .iter()
            .map(|x| &x.destination)
            .collect()
    }
}
