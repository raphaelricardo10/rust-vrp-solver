use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route::route_service::RouteService,
    solvers::solver::{Solution, Solver},
};

pub struct GreedySolver {
    solution: Solution,
    route_service: RouteService,
}

impl<'a> GreedySolver {
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

impl<'a> Solver<'a, GreedySolver> for GreedySolver {
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

    fn solve(&mut self) {
        self.route_service.assign_starting_points();
        self.run_all_iterations();
        self.solution = Self::map_solutions(&self.route_service);
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
}
