use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub struct Route {
    pub vehicle: Vehicle,
    stops: Vec<Stop>,
    distances: HashMap<(u32, u32), f64>,
}

impl Route {
    pub fn new(vehicle: Vehicle) -> Route {
        Route {
            vehicle,
            stops: Vec::new(),
            distances: HashMap::new(),
        }
    }

    fn generate_distances_key<'a>(stop_id1: &'a u32, stop_id2: &'a u32) -> (u32, u32) {
        let first_key = min(stop_id1, &stop_id2);
        let second_key = max(stop_id1, &stop_id2);

        (*first_key, *second_key)
    }

    pub fn add_stop(
        &mut self,
        stop: Stop,
        distances: HashMap<u32, f64>,
    ) -> Result<(), VehicleOverloadError> {
        if let Err(e) = self.vehicle.load(stop.usage) {
            return Err(e);
        }

        for stop_id in distances.keys() {
            let key = Route::generate_distances_key(stop_id, &stop.get_id());

            self.distances.insert(key, *distances.get(stop_id).unwrap());
        }

        self.stops.push(stop);

        Ok(())
    }

    pub fn total_distance(&self) -> f64 {
        let mut total: f64 = 0.0;

        for (prev_pos, stop) in self.stops[1..].iter().enumerate() {
            let key = Route::generate_distances_key(&stop.get_id(), &self.stops[prev_pos].get_id());

            total += self.distances.get(&key).unwrap();
        }

        total
    }
}
