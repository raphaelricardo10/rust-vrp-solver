use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::{solution::Solution, solver::Solver},
};

pub struct GreedySolver {
    pub solution: Solution,
    route_service: RouteService,
}

impl Solver for GreedySolver {
    fn solve(&mut self) {
        self.route_service.assign_starting_points();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.route_service.assign_stop_points();

        self.solution = Solution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        );
    }
}

impl GreedySolver {
    pub fn new(
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        stops: Vec<Stop>,
    ) -> GreedySolver {
        GreedySolver {
            solution: Solution::default(),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    fn run_iteration(&mut self) {
        let vehicle_ids: Vec<u32> = self
            .route_service
            .get_all_routes()
            .keys()
            .cloned()
            .collect();

        for vehicle_id in vehicle_ids {
            let stop_id = match self.route_service.get_nearest_stop(vehicle_id) {
                None => break,
                Some(stop) => stop.id,
            };

            self.route_service
                .assign_stop_to_route(vehicle_id, stop_id)
                .unwrap_or_else(|_| {
                    panic!("the vehicle {vehicle_id} should support the load from {stop_id}")
                });
        }
    }

    fn stop_condition_met(&self) -> bool {
        !self.route_service.has_available_stop()
    }
}
