use std::collections::HashMap;

use crate::domain::stop::Stop;

use super::distance_matrix::DistanceMatrixEntry;

pub type DistancesMatrixKey = (u32, u32);
pub type DistanceMatrixInput = HashMap<DistancesMatrixKey, f64>;
pub(crate) type DistanceMatrix<'a> = HashMap<DistancesMatrixKey, DistanceMatrixEntry<'a>>;

pub type StopsMap<'a> = HashMap<u32, &'a Stop>;

pub struct DistanceService<'a> {
    distances: DistanceMatrix<'a>,
}

impl<'a> DistanceService<'a> {
    pub fn new(stops: StopsMap<'a>, distances: &'a DistanceMatrixInput) -> DistanceService<'a> {
        let mut mapped_distances: DistanceMatrix = HashMap::new();

        for distance in distances {
            let source_stop_id = distance.0 .0;
            let destination_stop_id = distance.0 .1;
            let distance = distance.1;

            let source_stop = *stops.get(&source_stop_id).unwrap();
            let destination_stop = *stops.get(&source_stop_id).unwrap();

            let stop_distance =
                DistanceMatrixEntry::new(source_stop, destination_stop, *distance);

            mapped_distances.insert((source_stop_id, destination_stop_id), stop_distance);
        }

        DistanceService {
            distances: mapped_distances,
        }
    }

    pub fn get_distance(&self, from: &Stop, to: &Stop) -> Option<f64> {
        Some(self.distances.get(&(from.get_id(), to.get_id()))?.get_distance())
    }

    fn get_distances_from(&self, stop: &'a Stop) -> impl Iterator<Item = &DistanceMatrixEntry> {
        self.distances
            .iter()
            .filter(|x| x.0 .0 == stop.get_id())
            .map(|x| x.1)
    }

    pub fn get_nearest_stop(&self, stop: &'a Stop) -> Option<&Stop> {
        self.get_distances_from(stop)
            .min_by(|stop1, stop2| stop1.partial_cmp(stop2).unwrap())
            .map(|x| x.get_destination_stop())
    }

    pub fn get_k_nearest_stops(&'a self, stop: &'a Stop, k: usize) -> Vec<&Stop> {
        let mut stops = self
            .get_distances_from(stop)
            .collect::<Vec<&DistanceMatrixEntry>>();

        stops.sort_by(|stop1, stop2| stop1.partial_cmp(stop2).unwrap());

        stops[0..k]
            .iter()
            .map(|x| x.get_destination_stop())
            .collect()
    }
}
