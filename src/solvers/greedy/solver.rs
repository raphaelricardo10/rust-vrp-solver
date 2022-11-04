use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route_service::RouteService,
};

pub type Solution = HashMap<u32, Vec<u32>>;

pub struct GreedySolver<'a> {
    solution: Solution,
    route_service: RouteService<'a>,
}

impl<'a> GreedySolver<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> GreedySolver<'a> {
        GreedySolver {
            solution: HashMap::new(),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    fn get_all_vehicle_ids(&self) -> Vec<u32> {
        self.route_service
            .get_vehicles()
            .iter()
            .map(|x| x.get_id())
            .collect()
    }

    fn construct_solutions(&mut self) {
        for vehicle in self.route_service.get_vehicles().iter() {
            let vehicle_id = vehicle.get_id();

            let solution = self
                .route_service
                .get_route(vehicle.get_id())
                .unwrap()
                .get_stops()
                .iter()
                .map(|x| x.get_id())
                .collect();

            self.solution.insert(vehicle_id, solution);
        }
    }

    fn construct_routes_in_parallel(&mut self, vehicle_ids: &Vec<u32>) {
        for vehicle_id in vehicle_ids.iter() {
            let stop_id = match self.route_service.get_nearest_stop(*vehicle_id) {
                None => break,
                Some(stop) => stop.get_id(),
            };

            self.route_service
                .assign_stop_to_route(*vehicle_id, stop_id)
                .unwrap();
        }
    }

    fn construct_all_routes(&mut self) {
        let vehicle_ids: Vec<u32> = self.get_all_vehicle_ids();

        while !self.route_service.has_available_stop() {
            self.construct_routes_in_parallel(&vehicle_ids);
        }
    }

    pub fn solution_total_distance(&self) -> f64 {
        self.route_service.total_distance()
    }

    pub fn solve(&mut self) {
        self.route_service.assign_starting_points();
        self.construct_all_routes();
        self.construct_solutions();
    }

    pub fn get_solution(&self) -> &Solution {
        &self.solution
    }
}
