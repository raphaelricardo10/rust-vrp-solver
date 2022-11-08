use std::collections::HashMap;

use crate::{
    services::route::route_service::RouteService,
};

pub type Solution = HashMap<u32, Vec<u32>>;

pub trait Solver<'a, T> {
    fn solve(&mut self);
    fn run_iteration(&mut self);
    fn stop_condition_met(&self) -> bool;
    fn get_solution(&self) -> &Solution;
    fn solution_total_distance(&self) -> f64;

    fn get_all_vehicle_ids(route_service: &RouteService) -> Vec<u32> {
        route_service
            .get_vehicles()
            .iter()
            .map(|x| x.get_id())
            .collect()
    }

    fn construct_solutions(route_service: &RouteService) -> Solution {
        route_service
            .get_all_routes()
            .iter()
            .map(|(vehicle_id, route)| -> (u32, Vec<u32>) {
                (
                    *vehicle_id,
                    route.get_stops().iter().map(|stop| stop.get_id()).collect(),
                )
            })
            .collect()
    }

    fn run_all_iterations(&mut self) {
        while !self.stop_condition_met() {
            self.run_iteration();
        }
    }
}
