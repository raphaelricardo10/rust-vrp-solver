use rand::{seq::SliceRandom, Rng};
use rand::thread_rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    local_search::two_opt::TwoOptSearcher,
    services::{distance::distance_service::DistanceMatrix, route::route_service::RouteService},
    solvers::solution::Solution,
};

pub struct GraspSolver<'a, R: Rng + ?Sized> {
    rng: &'a mut R,
    rcl_size: usize,
    pub solution: Solution,
    local_search: TwoOptSearcher,
    route_service: RouteService,
    max_improvement_times: u8,
    times_without_improvement: u8,
}

impl<'a, R: Rng + ?Sized> GraspSolver<'a, R> {
    pub fn new(
        rcl_size: usize,
        vehicles: Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        max_improvement_times: u8,
        stops: Vec<Stop>,
        rng: &'a mut R,
    ) -> Self {
        Self {
            rng,
            rcl_size,
            max_improvement_times,
            solution: Solution::default(),
            times_without_improvement: Default::default(),
            local_search: TwoOptSearcher::new(stops.clone(), distances),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    pub fn solve(&mut self) {
        while !self.stop_condition_met() {
            self.run_generation();
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.times_without_improvement >= self.max_improvement_times
    }

    fn run_generation(&mut self) {
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

    fn generate_solution(&mut self, vehicle_ids: &Vec<u32>) {
        self.route_service.reset();
        self.route_service.assign_starting_points();

        while self.route_service.has_available_stop().unwrap() {
            self.run_iteration(vehicle_ids)
        }

        self.route_service.assign_stop_points();
    }

    fn run_iteration(&mut self, vehicle_ids: &Vec<u32>) {
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

    pub fn get_random_near_stop(&mut self, vehicle_id: u32) -> Option<&Stop> {
        let near_stops = self
            .route_service
            .get_k_nearest_stops(vehicle_id, self.rcl_size)?;
        let chosen = *near_stops.choose(self.rng)?;

        Some(chosen)
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
}
