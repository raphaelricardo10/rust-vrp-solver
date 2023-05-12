use std::rc::Rc;

use rand::{seq::SliceRandom, Rng};

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    local_search::two_opt::TwoOptSearcher,
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{solver::Solver, vrp_solution::VrpSolution},
};

#[repr(C)]
pub struct GraspSolverParameters {
    pub rcl_size: usize,
    pub max_improvement_times: u8,
}

pub struct VrpGraspSolver<R: Rng + ?Sized> {
    rng: Box<R>,
    solution: VrpSolution,
    route_service: RouteService,
    local_search: TwoOptSearcher,
    times_without_improvement: u8,
    parameters: GraspSolverParameters,
}

impl<R: Rng + ?Sized> Solver<VrpSolution> for VrpGraspSolver<R> {
    fn solve(&mut self) -> VrpSolution {
        while !self.stop_condition_met() {
            self.run_generation();
        }

        let solution = self.solution.clone();
        self.solution = Default::default();
        solution
    }
}

impl<R: Rng + ?Sized> VrpGraspSolver<R> {
    pub fn new(
        stops: Vec<Stop>,
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        parameters: GraspSolverParameters,
        rng: Box<R>,
    ) -> Self {
        let distance_service = Rc::new(DistanceService::new(stops.clone(), distances));

        Self {
            rng,
            parameters,
            solution: Default::default(),
            times_without_improvement: Default::default(),
            local_search: TwoOptSearcher::new(distance_service.clone()),
            route_service: RouteService::new(stops, vehicles, distance_service),
        }
    }

    fn stop_condition_met(&self) -> bool {
        self.times_without_improvement >= self.parameters.max_improvement_times
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

        let solution = VrpSolution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        );

        match self.should_update_solution(&solution) {
            true => {
                self.solution = solution;
                self.times_without_improvement = 0;
            }
            false => self.times_without_improvement += 1,
        }
    }

    fn generate_solution(&mut self, vehicle_ids: &Vec<u32>) {
        self.route_service.reset();
        self.route_service.assign_starting_points();

        while self.route_service.has_available_stop() {
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
                .unwrap_or_else(|_| panic!("the vehicle {vehicle_id} should support the load"));

            let route = self.route_service.get_route_mut(*vehicle_id);
            self.local_search.run(route);
        }
    }

    pub fn get_random_near_stop(&mut self, vehicle_id: u32) -> Option<&Stop> {
        let near_stops = self
            .route_service
            .get_k_nearest_stops(vehicle_id, self.parameters.rcl_size)?;
        let chosen = *near_stops.choose(&mut self.rng)?;

        Some(chosen)
    }

    fn run_local_search(&mut self, vehicle_ids: &Vec<u32>) {
        for vehicle_id in vehicle_ids {
            let route = self.route_service.get_route_mut(*vehicle_id);

            self.local_search.run(route);
        }
    }

    fn should_update_solution(&self, solution: &VrpSolution) -> bool {
        solution.is_better_than(&self.solution)
    }
}
