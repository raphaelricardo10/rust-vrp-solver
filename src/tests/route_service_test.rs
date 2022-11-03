use std::collections::HashMap;

use crate::{domain::{vehicle::Vehicle, stop::Stop, route::DistanceMatrix}, services::route_service::RouteService};

#[test]
fn route_service_started() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    vehicles.push(Vehicle::new(5));
    vehicles.push(Vehicle::new(10));

    let mut stops: Vec<Stop> = Vec::new();
    stops.push(Stop::new(0, 2));
    stops.push(Stop::new(1, 3));

    let distances: DistanceMatrix = HashMap::new();

    let route_service = RouteService::new(&mut vehicles, &distances, &stops);

    assert_eq!(route_service.get_available_stops().len(), 2);
    assert_eq!(route_service.get_routes().len(), 2);
    assert_eq!(route_service.get_vehicles().len(), 2);
}