use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route_service::RouteService,
};

pub type Solution = HashMap<u32, Vec<u32>>;

pub trait Solver<'a, T> {
    fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> T;

    fn solve(&mut self);
    fn get_solution(&self) -> &Solution;
    fn solution_total_distance(&self) -> f64;
    fn construct_routes_in_parallel(route_service: &mut RouteService, vehicle_ids: &Vec<u32>);

    fn get_all_vehicle_ids(route_service: &RouteService) -> Vec<u32> {
        route_service
            .get_vehicles()
            .iter()
            .map(|x| x.get_id())
            .collect()
    }

    fn construct_solutions(route_service: &RouteService) -> Solution{
        let mut solution: Solution = HashMap::new();

        for vehicle in route_service.get_vehicles().iter() {
            let vehicle_id = vehicle.get_id();

            let route = route_service
                .get_route(vehicle.get_id())
                .unwrap()
                .get_stops()
                .iter()
                .map(|x| x.get_id())
                .collect();

            solution.insert(vehicle_id, route);
        }

        solution
    }

    fn construct_all_routes(route_service: &mut RouteService) {
        let vehicle_ids: Vec<u32> = Self::get_all_vehicle_ids(route_service);

        while route_service.has_available_stop() {
            Self::construct_routes_in_parallel(route_service, &vehicle_ids);
        }
    }
}
