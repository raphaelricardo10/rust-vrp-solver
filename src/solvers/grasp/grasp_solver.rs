use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    local_search::two_opt::TwoOptSearcher,
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::solution::Solution,
};

pub struct GraspSolver {
    rcl_size: usize,
    pub solution: Solution,
    local_search: TwoOptSearcher,
    route_service: RouteService,
    times_without_improvement: u8,
}

impl<'a> GraspSolver {
    pub fn new(
        rcl_size: usize,
        vehicles: Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: Vec<Stop>,
    ) -> GraspSolver {
        GraspSolver {
            rcl_size,
            solution: Solution::default(),
            times_without_improvement: u8::default(),
            local_search: TwoOptSearcher::new(stops.clone(), distances),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    pub fn get_random_near_stop(&self, vehicle_id: u32) -> Option<&Stop> {
        let near_stops = self
            .route_service
            .get_k_nearest_stops(vehicle_id, self.rcl_size)?;
        let chosen = *near_stops.choose(&mut thread_rng())?;

        Some(chosen)
    }

    fn generate_solution(&mut self, vehicle_ids: &Vec<u32>) {
        self.route_service.reset();
        self.route_service.assign_starting_points();

        while self.route_service.has_available_stop().unwrap() {
            for vehicle_id in vehicle_ids {
                let stop_id = match self.get_random_near_stop(*vehicle_id) {
                    None => break,
                    Some(stop) => stop.id,
                };

                self.route_service
                    .assign_stop_to_route(*vehicle_id, stop_id)
                    .unwrap();

                let route = self.route_service.get_route_mut(*vehicle_id).unwrap();
                self.local_search.run(route);
            }
        }

        self.route_service.assign_stop_points();
    }

    fn run_local_search(&mut self, vehicle_ids: &Vec<u32>) {
        for vehicle_id in vehicle_ids {
            let route = self.route_service.get_route_mut(*vehicle_id).unwrap();

            self.local_search.run(route);
        }
    }

    fn should_update_solution(&self, solution: &Solution) -> bool {
        solution.is_better_than(&self.solution)
    }

    fn run_iteration(&mut self) {
        let vehicle_ids: Vec<u32> = self
            .route_service
            .get_all_routes()
            .keys()
            .cloned()
            .collect();

        self.generate_solution(&vehicle_ids);
        self.run_local_search(&vehicle_ids);

        let solution = Solution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        );

        if self.should_update_solution(&solution) {
            self.solution = solution;
            self.times_without_improvement = 0;
        } else {
            self.times_without_improvement += 1;
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.times_without_improvement >= 3
    }

    pub fn solve(&mut self) {
        while !self.stop_condition_met() {
            self.run_iteration();
        }
    }
}
