use rstest::fixture;

use std::collections::HashMap;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

pub type VehicleFactory = fn(number: u32) -> Vec<Vehicle>;

#[fixture]
pub fn distances() -> DistanceMatrix {
    HashMap::from([
        ((0, 1), 2.0),
        ((0, 2), 1.0),
        ((0, 3), 3.0),
        ((0, 4), 0.5),
        ((1, 0), 2.0),
        ((1, 2), 5.0),
        ((1, 3), 3.0),
        ((1, 4), 5.0),
        ((2, 0), 1.0),
        ((2, 1), 5.0),
        ((2, 3), 2.0),
        ((2, 4), 2.0),
        ((3, 0), 3.0),
        ((3, 1), 3.0),
        ((3, 2), 2.0),
        ((3, 4), 5.0),
        ((4, 0), 0.5),
        ((4, 1), 5.0),
        ((4, 2), 2.0),
        ((4, 3), 5.0),
    ])
}

#[fixture]
pub fn stops() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 0),
        Stop::new(1, 0),
        Stop::new(2, 0),
        Stop::new(3, 0),
        Stop::new(4, 100),
    ])
}

#[fixture]
pub fn full_stops() -> Vec<Stop> {
    Vec::from([Stop::new(0, 5), Stop::new(1, 100)])
}

#[fixture]
pub fn stops_with_crossings() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 10),
        Stop::new(3, 10),
        Stop::new(4, 10),
        Stop::new(1, 10),
        Stop::new(2, 10),
        Stop::new(0, 10),
    ])
}

#[fixture]
pub fn vehicle_factory() -> VehicleFactory {
    fn wrapper(number: u32) -> Vec<Vehicle> {
        let mut vehicles = Vec::new();

        for i in 0..number {
            vehicles.push(Vehicle::new(i, 10));
        }

        vehicles
    }

    wrapper
}

#[fixture]
pub fn distance_service(distances: DistanceMatrix, stops: Vec<Stop>) -> DistanceService {
    DistanceService::new(stops, &distances)
}
