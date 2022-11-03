use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};
use std::{
    collections::HashMap,
};

pub type DistanceMatrix = HashMap<(u32, u32), f64>;
pub type DistanceMatrixLine = HashMap<u32, f64>;

pub struct Route<'a> {
    stops: Vec<Stop>,
    vehicle: &'a mut Vehicle,
    distances: &'a DistanceMatrix,
}

impl<'a> Route<'a> {
    pub fn new(vehicle: &'a mut Vehicle, distances: &'a DistanceMatrix) -> Route<'a> {
        Route {
            vehicle,
            distances,
            stops: Vec::new(),
        }
    }

    pub fn add_stop(&mut self, stop: Stop) -> Result<(), VehicleOverloadError> {
        if let Err(e) = self.vehicle.load(stop.usage) {
            return Err(e);
        }

        self.stops.push(stop);

        Ok(())
    }

    pub fn total_distance(&self) -> f64 {
        let mut total: f64 = 0.0;

        for (prev_pos, stop) in self.stops[1..].iter().enumerate() {
            let prev_stop_id = self.stops[prev_pos].get_id();
            total += self.distances.get(&(prev_stop_id, stop.get_id())).unwrap();
        }

        total
    }
}
