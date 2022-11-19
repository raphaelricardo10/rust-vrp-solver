use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};

#[derive(Clone)]
pub struct Route {
    pub stops: Vec<Stop>,
    pub vehicle: Vehicle,
    total_distance: f64,
}

impl Route {
    pub fn new(vehicle: Vehicle) -> Route {
        Route {
            vehicle,
            stops: Vec::new(),
            total_distance: f64::default(),
        }
    }

    pub fn reset(&mut self) {
        self.stops.clear();
        self.total_distance = Default::default()
    }

    pub fn get_current_stop(&self) -> Option<&Stop> {
        self.stops.last()
    }

    pub fn can_add_stop(&self, stop: &Stop) -> bool {
        self.vehicle.can_support_load(stop.usage)
    }

    pub fn add_stop(&mut self, stop: Stop, distance: f64) -> Result<(), VehicleOverloadError> {
        self.vehicle.load(stop.usage)?;

        self.stops.push(stop);
        self.total_distance += distance;

        Ok(())
    }

    pub fn total_distance(&self) -> f64 {
        self.total_distance
    }

    pub fn swap_stops(&mut self, index1: usize, index2: usize) {
        self.stops.swap(index1, index2);
    }
}
