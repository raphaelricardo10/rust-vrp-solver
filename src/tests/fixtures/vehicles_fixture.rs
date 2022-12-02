use rstest::fixture;

use crate::domain::vehicle::Vehicle;

pub type VehicleFactory = fn(number: u32) -> Vec<Vehicle>;

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
