use rstest::fixture;

use std::collections::HashMap;

use crate::domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle};

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
        ((4, 0), 1.0),
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
