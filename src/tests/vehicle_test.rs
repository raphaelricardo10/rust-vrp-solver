use crate::domain::vehicle::Vehicle;

#[test]
fn cannot_overload_vehicle() {
    let mut vehicle = Vehicle::new(0, 10);

    vehicle.load(5).unwrap();

    if vehicle.load(40).is_ok() {
        panic!();
    }
}

#[test]
fn cannot_have_negative_usage() {
    let mut vehicle = Vehicle::new(0, 10);

    if vehicle.unload(10).is_ok() {
        panic!();
    }
}
