use crate::domain::vehicle::Vehicle;

#[test]
fn cannot_overload_vehicle() {
    let mut vehicle = Vehicle::new(0, 10);

    vehicle.load(5).unwrap();

    match vehicle.load(40) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn cannot_have_negative_usage() {
    let mut vehicle = Vehicle::new(0, 10);

    match vehicle.unload(10) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
