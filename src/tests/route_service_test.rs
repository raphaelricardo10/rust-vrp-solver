use std::collections::HashMap;

use crate::{domain::{vehicle::Vehicle, stop::Stop, route::DistanceMatrix}, services::route_service::RouteService};

#[test]
fn route_service_started() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    vehicles.push(Vehicle::new(0, 5));
    vehicles.push(Vehicle::new(1, 10));

    let mut stops: Vec<Stop> = Vec::new();
    stops.push(Stop::new(0, 2));
    stops.push(Stop::new(1, 3));

    let distances: DistanceMatrix = HashMap::new();

    let route_service = RouteService::new(&mut vehicles, &distances, &stops);

    assert_eq!(route_service.get_available_stops().len(), 2);
    assert_eq!(route_service.get_all_routes().len(), 2);
    assert_eq!(route_service.get_vehicles().len(), 2);
}

#[test]
fn can_assign_stop_to_route() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    vehicles.push(Vehicle::new(0, 5));
    vehicles.push(Vehicle::new(1, 10));

    let mut stops: Vec<Stop> = Vec::new();
    stops.push(Stop::new(0, 2));
    stops.push(Stop::new(1, 3));

    let distances: DistanceMatrix = HashMap::new();

    let mut route_service = RouteService::new(&mut vehicles, &distances, &stops);

    route_service.assign_stop_to_route(0, 0);

    assert_eq!(route_service.get_route(0).get_stops().len(), 1)
}

#[test]
fn can_get_nearest_stop() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    vehicles.push(Vehicle::new(0, 5));

    let mut stops: Vec<Stop> = Vec::new();
    stops.push(Stop::new(0, 0));
    stops.push(Stop::new(1, 0));
    stops.push(Stop::new(2, 0));
    stops.push(Stop::new(3, 0));

    let mut distances: DistanceMatrix = HashMap::new();
    distances.insert((0, 1), 2.0);
    distances.insert((0, 2), 1.0);
    distances.insert((0, 3), 3.0);

    let route_service = RouteService::new(&mut vehicles, &distances, &stops);

    assert_eq!(route_service.get_nearest_stop(0).get_id(), 2);
}