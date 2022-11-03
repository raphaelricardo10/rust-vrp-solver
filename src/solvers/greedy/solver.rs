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

    fn construct_solutions(&mut self) {
        for vehicle in self.route_service.get_vehicles().iter(){
            let vehicle_id = vehicle.get_id();
            
            let solution = self.route_service
                .get_route(vehicle.get_id())
                .get_stops()
                .iter()
                .map(|x| x.get_id())
                .collect();

            self.solution.insert(vehicle_id, solution);
        }
    }

    pub fn solve(&mut self) {
        let vehicle_id = self.route_service.get_vehicles().first().unwrap().get_id();

        self.route_service.assign_stop_to_route(vehicle_id, 0);

        while !self.route_service.has_available_stop() {
            let stop_id = self.route_service.get_nearest_stop(vehicle_id).get_id();

            self.route_service.assign_stop_to_route(vehicle_id, stop_id);
        }

        self.construct_solutions();
    }

    pub fn get_solution(&self) -> &Solution{
        &self.solution
    }
}
