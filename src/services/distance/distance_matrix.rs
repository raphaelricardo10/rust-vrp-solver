use std::{cmp::Ordering, collections::HashMap};

use crate::domain::stop::Stop;

pub type DistanceMatrixKey<T> = (T, T);
pub type DistanceMatrixMap<K, V> = HashMap<DistanceMatrixKey<K>, V>;

pub (crate) struct DistanceMatrixEntry<'a> {
    distance: f64,
    source: &'a Stop,
    destination: &'a Stop,
}

impl<'a> DistanceMatrixEntry<'a> {
    pub(crate) fn new(source: &'a Stop, destination: &'a Stop, distance: f64) -> DistanceMatrixEntry {
        DistanceMatrixEntry {
            source,
            distance,
            destination,
        }
    }

    pub(crate) fn get_distance(&self) -> f64 {
        self.distance
    }

    pub(crate) fn get_source_route(&'a self) -> &'a Stop {
        self.source
    }

    pub(crate) fn get_destination_route(&'a self) -> &'a Stop {
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

pub struct DistanceMatrix<K, V> {
    pub(crate) entries: DistanceMatrixMap<K, V>,
}
