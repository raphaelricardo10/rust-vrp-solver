use std::cmp::Ordering;

use crate::domain::stop::Stop;

pub(crate) struct DistanceMatrixEntry {
    distance: f64,
    destination: Stop,
}

impl DistanceMatrixEntry {
    pub(crate) fn new(
        destination: Stop,
        distance: f64,
    ) -> DistanceMatrixEntry {
        DistanceMatrixEntry {
            distance,
            destination,
        }
    }

    pub(crate) fn get_distance(&self) -> f64 {
        self.distance
    }

    pub(crate) fn get_destination_stop(&self) -> &Stop {
        &self.destination
    }
}

impl<'a> PartialOrd for DistanceMatrixEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_distance().partial_cmp(&other.get_distance())
    }
}

impl<'a> PartialEq for DistanceMatrixEntry {
    fn eq(&self, other: &Self) -> bool {
        self.get_distance() == other.get_distance()
    }
}

impl<'a> Eq for DistanceMatrixEntry {}
