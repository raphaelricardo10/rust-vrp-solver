use std::cmp::Ordering;

use crate::domain::stop::Stop;

pub(crate) struct DistanceMatrixEntry<'a> {
    distance: f64,
    source: &'a Stop,
    destination: &'a Stop,
}

impl<'a> DistanceMatrixEntry<'a> {
    pub(crate) fn new(
        source: &'a Stop,
        destination: &'a Stop,
        distance: f64,
    ) -> DistanceMatrixEntry {
        DistanceMatrixEntry {
            source,
            distance,
            destination,
        }
    }

    pub(crate) fn get_distance(&self) -> f64 {
        self.distance
    }

    pub(crate) fn get_source_stop(&'a self) -> &'a Stop {
        self.source
    }

    pub(crate) fn get_destination_stop(&'a self) -> &'a Stop {
        self.destination
    }
}

impl<'a> PartialOrd for DistanceMatrixEntry<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_distance().partial_cmp(&other.get_distance())
    }
}

impl<'a> PartialEq for DistanceMatrixEntry<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.get_distance() == other.get_distance()
    }
}

impl<'a> Eq for DistanceMatrixEntry<'a> {}
