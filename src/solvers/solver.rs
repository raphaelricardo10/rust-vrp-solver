use crate::services::route::route_service::RouteService;

pub trait Solver<T> {
    fn solve(&mut self);
    fn run_iteration(&mut self);
    fn stop_condition_met(&self) -> bool;
    fn solution_total_distance(&self) -> f64;
    fn get_route_service(&mut self) -> &mut RouteService;
}
