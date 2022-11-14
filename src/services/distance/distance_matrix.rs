use std::cmp::Ordering;

use crate::domain::stop::Stop;

pub(crate) struct DistanceMatrixEntry {
    pub(crate) distance: f64,
    pub(crate) destination: Stop,
}

impl DistanceMatrixEntry {
    pub(crate) fn new(destination: Stop, distance: f64) -> DistanceMatrixEntry {
        DistanceMatrixEntry {
            distance,
            destination,
        }
    }
}

impl PartialOrd for DistanceMatrixEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for DistanceMatrixEntry {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for DistanceMatrixEntry {}
