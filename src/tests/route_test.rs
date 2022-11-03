use std::collections::HashMap;

use crate::domain::{vehicle::Vehicle, route::{DistanceMatrix, Route}, stop::Stop};

#[test]
fn route_distance_calculation() {
    let mut vehicle = Vehicle::new(0, 10);
    
    let mut distances: DistanceMatrix = HashMap::new();
    distances.insert((0, 1), 20.0);
    distances.insert((1, 0), 18.0);

    let mut route = Route::new(&mut vehicle, &distances);
    
    let stop1 = Stop::new(0, 4);
    let stop2 = Stop::new(1, 5);

    route.add_stop(stop1).unwrap();
    route.add_stop(stop2).unwrap();

    assert_eq!(route.total_distance(), 20.0);
}

#[test]
fn route_cannot_overload_vehicle() {
    let mut vehicle = Vehicle::new(0, 10);
    let distances: DistanceMatrix = HashMap::new();

    let mut route = Route::new(&mut vehicle, &distances);

    let stop1 = Stop::new(0, 4);
    let stop2 = Stop::new(1, 10);

    route.add_stop(stop1).unwrap();
    
    match route.add_stop(stop2) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}