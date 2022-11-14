use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::solution::Solution,
};

pub struct GreedySolver {
    pub solution: Solution,
    route_service: RouteService,
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
                .unwrap();
        }
    }
    
    pub fn solve(&mut self) {
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
    
    fn solution_total_distance(&self) -> f64 {
        self.solution.total_distance
    }
    
    fn stop_condition_met(&self) -> bool {
        !self.route_service.has_available_stop().unwrap()
    }
    
    fn get_route_service(&mut self) -> &mut RouteService {
        &mut self.route_service
    }
}
