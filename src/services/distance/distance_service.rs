use std::collections::HashMap;

use crate::domain::stop::Stop;

use super::distance_matrix::{DistanceMatrix, DistanceMatrixEntry, DistanceMatrixMap};

impl<'a> DistanceMatrix<u32, &DistanceMatrixEntry<'a>> {
    pub fn new(
        entries: DistanceMatrixMap<&'a Stop, f64>,
    ) -> DistanceMatrix<u32, DistanceMatrixEntry> {
        let mut distance_matrix: DistanceMatrixMap<u32, DistanceMatrixEntry> = HashMap::new();

        for entry in entries {
            let source_stop: &Stop = entry.0 .0;
            let destination_stop: &Stop = entry.0 .1;
            let distance: f64 = entry.1;

            let stop_distance = DistanceMatrixEntry::new(source_stop, destination_stop, distance);

            distance_matrix.insert(
                (source_stop.get_id(), destination_stop.get_id()),
                stop_distance,
            );
        }

        DistanceMatrix {
            entries: distance_matrix,
        }
    }

    fn get_distances_from(&self, stop: &'a Stop) -> impl Iterator<Item = &&DistanceMatrixEntry> {
        self.entries
            .iter()
            .filter(|x| x.0 .0 == stop.get_id())
            .map(|x| x.1)
    }

    pub fn get_nearest_stop(&self, stop: &'a Stop) -> Option<&Stop> {
        self.get_distances_from(stop)
            .min_by(|stop1, stop2| stop1.partial_cmp(stop2).unwrap())
            .map(|x| x.get_destination_stop())
    }
}
