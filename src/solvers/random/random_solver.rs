use std::rc::Rc;

use rand::Rng;

use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::{solver::Solver, vrp_solution::VrpSolution},
};

pub struct RandomSolver<R: Rng + ?Sized> {
    rng: Box<R>,
    route_service: RouteService,
}

impl<R: Rng + ?Sized> Solver for RandomSolver<R> {
    type ConcreteSolution = VrpSolution;

    fn solve(&mut self) -> VrpSolution {
        let vehicle_ids: Vec<u32> = self
            .route_service
            .get_vehicles()
            .iter()
            .map(|vehicle| vehicle.id)
            .collect();

        self.route_service.assign_starting_points();

        while self.route_service.has_available_stop() {
            for vehicle_id in vehicle_ids.iter() {
                let stop = match self
                    .route_service
                    .get_random_stop(*vehicle_id, &mut self.rng)
                {
                    Some(stop) => stop,
                    None => continue,
                };

                self.route_service
                    .assign_stop_to_route(*vehicle_id, stop.id)
                    .unwrap_or_else(|_| panic!("the vehicle {vehicle_id} should support the load"));
            }
        }

        self.route_service.assign_stop_points();

        let solution = VrpSolution::new(
            self.route_service.get_all_routes(),
            self.route_service.total_distance(),
        );

        self.route_service.reset();

        solution
    }
}

impl<R: Rng + ?Sized> RandomSolver<R> {
    pub fn new(
        stops: Vec<Stop>,
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        rng: Box<R>,
    ) -> Self {
        let distance_service = Rc::new(DistanceService::new(stops.clone(), distances));
        let route_service = RouteService::new(stops, vehicles, distance_service);

        Self { rng, route_service }
    }
}
