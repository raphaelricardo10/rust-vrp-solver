use std::collections::HashMap;

use crate::services::route::route_service::RouteService;

pub type Solution = HashMap<u32, Vec<u32>>;

pub trait Solver<T> {
    fn run_iteration(&mut self);
    fn stop_condition_met(&self) -> bool;
    fn get_solution(&self) -> &Solution;
    fn set_solution(&mut self, solution: Solution);
    fn solution_total_distance(&self) -> f64;
    fn get_route_service(&mut self) -> &mut RouteService;

    fn get_all_vehicle_ids(route_service: &RouteService) -> Vec<u32> {
        route_service
            .get_vehicles()
            .iter()
            .map(|x| x.get_id())
            .collect()
    }

    fn map_solutions(&mut self) -> Solution {
        self.get_route_service()
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

    fn solve(&mut self) {
        self.get_route_service().assign_starting_points();

        while !self.stop_condition_met() {
            self.run_iteration();
        }

        self.get_route_service().assign_stop_points();

        let solution = self.map_solutions();
        self.set_solution(solution);
    }
}
