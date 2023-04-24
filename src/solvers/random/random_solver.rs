use std::rc::Rc;

use rand::Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{solution::Solution, solver::Solver},
};

pub struct RandomSolver<'a, R: Rng + ?Sized> {
    pub solution: Solution,
    rng: &'a mut R,
    route_service: RouteService,
}

impl<'a, R: Rng + ?Sized> Solver for RandomSolver<'a, R> {
    fn solve(&mut self) {
        let vehicle_ids: Vec<u32> = self
            .route_service
            .get_vehicles()
            .iter()
            .map(|vehicle| vehicle.id)
            .collect();

        self.route_service.assign_starting_points();

        while self.route_service.has_available_stop() {
            for vehicle_id in vehicle_ids.iter() {
                let stop = match self.route_service.get_random_stop(*vehicle_id, self.rng) {
                    Some(stop) => stop,
                    None => continue,
                };

                self.route_service
                    .assign_stop_to_route(*vehicle_id, stop.id)
                    .unwrap_or_else(|_| panic!("the vehicle {vehicle_id} should support the load"));
            }
        }

        self.route_service.assign_stop_points();

        self.solution = Solution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        );
    }

    fn reset_solution(&mut self) {
        self.solution = Default::default();
    }

    fn get_solution(&self) -> &Solution {
        &self.solution
    }
}

impl<'a, R: Rng + ?Sized> RandomSolver<'a, R> {
    pub fn new(
        stops: Vec<Stop>,
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        rng: &'a mut R,
    ) -> Self {
        let distance_service = Rc::new(DistanceService::new(stops.clone(), distances));
        let route_service = RouteService::new(stops, vehicles, distance_service);

        Self {
            rng,
            route_service,
            solution: Default::default(),
        }
    }
}
