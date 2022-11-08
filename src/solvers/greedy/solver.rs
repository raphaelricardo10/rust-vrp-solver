use std::collections::HashMap;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{
        distance::distance_service::DistanceMatrix, route::route_service::RouteService,
    },
    solvers::solver::{Solution, Solver},
};

pub struct GreedySolver {
    solution: Solution,
    route_service: RouteService,
}

impl GreedySolver {
    pub fn new(
        vehicles: Vec<Vehicle>,
        distances: DistanceMatrix,
        stops: Vec<Stop>,
    ) -> GreedySolver {
        GreedySolver {
            solution: HashMap::new(),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }
}

impl Solver<GreedySolver> for GreedySolver {
    fn run_iteration(&mut self) {
        let vehicle_ids = Self::get_all_vehicle_ids(&self.route_service);

        for vehicle_id in vehicle_ids {
            let stop_id = match self.route_service.get_nearest_stop(vehicle_id) {
                None => break,
                Some(stop) => stop.get_id(),
            };

            self.route_service
                .assign_stop_to_route(vehicle_id, stop_id)
                .unwrap();
        }
    }

    fn get_solution(&self) -> &Solution {
        &self.solution
    }

    fn solution_total_distance(&self) -> f64 {
        self.route_service.total_distance()
    }

    fn stop_condition_met(&self) -> bool {
        !self.route_service.has_available_stop().unwrap()
    }

    fn set_solution(&mut self, solution: Solution) {
        self.solution = solution
    }

    fn get_route_service(&mut self) -> &mut RouteService {
        &mut self.route_service
    }
}
