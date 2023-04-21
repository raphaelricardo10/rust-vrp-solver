use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};

#[repr(C)]
#[derive(Clone)]
pub struct Route {
    pub stops: Vec<Stop>,
    pub vehicle: Vehicle,
    total_distance: f32,
}

impl Route {
    pub fn new(vehicle: Vehicle) -> Route {
        Route {
            vehicle,
            stops: Vec::new(),
            total_distance: f32::default(),
        }
    }

    pub fn reset(&mut self) {
        self.stops.clear();
        self.vehicle.reset();
        self.total_distance = Default::default()
    }

    pub fn get_current_stop(&self) -> Option<&Stop> {
        self.stops.last()
    }

    pub fn can_add_stop(&self, stop: &Stop) -> bool {
        self.vehicle.can_support_load(stop.usage)
    }

    pub fn add_stop(&mut self, stop: Stop, distance: f32) -> Result<(), VehicleOverloadError> {
        self.vehicle.load(stop.usage)?;

        self.stops.push(stop);
        self.total_distance += distance;

        Ok(())
    }

    pub fn remove_stop(&mut self, stop_index: usize, distance_reduction: f32) {
        self.stops.remove(stop_index);
        self.total_distance -= distance_reduction;
    }

    pub fn add_stop_at(&mut self, stop: Stop, index: usize, distance_change: f32) {
        self.stops.insert(index, stop);
        self.total_distance += distance_change;
    }

    pub fn add_multiple_stops_at(&mut self, stops: Vec<Stop>, index: usize, distance_change: f32) {
        self.stops.splice(index..index, stops);

        if index == 0 {
            self.stops.insert(
                0,
                *self.stops.last().expect("the route should not be empty"),
            );
        }

        self.total_distance += distance_change;
    }

    pub fn total_distance(&self) -> f32 {
        self.total_distance
    }

    pub fn swap_stops(&mut self, index1: usize, index2: usize, distance_change: f32) {
        self.stops.swap(index1, index2);
        self.total_distance += distance_change;
    }
}
