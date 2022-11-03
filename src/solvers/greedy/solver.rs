use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route_service::RouteService,
};

pub struct GreedySolver<'a> {
    route_service: RouteService<'a>,
    solution: Vec<u32>,
}

impl<'a> GreedySolver<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> GreedySolver {
        GreedySolver {
            route_service: RouteService::new(vehicles, distances, stops),
            solution: Vec::new(),
        }
    }

    fn construct_solution(&self, vehicle_id: u32) -> Vec<u32> {
        self.route_service
            .get_route(vehicle_id)
            .get_stops()
            .iter()
            .map(|x| x.get_id())
            .collect()
    }

    pub fn solve(&mut self) {
        let vehicle_id = self.route_service.get_vehicles().first().unwrap().get_id();

        self.route_service.assign_stop_to_route(vehicle_id, 0);

        while !self.route_service.has_available_stop() {
            let stop_id = self.route_service.get_nearest_stop(vehicle_id).get_id();

            self.route_service.assign_stop_to_route(vehicle_id, stop_id);
        }

        self.solution = self.construct_solution(vehicle_id)
    }

    pub fn get_solution(&self) -> &Vec<u32> {
        &self.solution
    }
}
