use rstest::fixture;

use std::collections::HashMap;

use crate::{
    domain::{route::Route, stop::Stop, vehicle::Vehicle},
    local_search::two_opt::TwoOptSearcher,
    services::{
        distance::distance_service::{DistanceMatrix, DistanceService},
        route::route_service::RouteService,
    },
    solvers::greedy::greedy_solver::GreedySolver,
    stop_swapper::StopSwapper,
};

pub type VehicleFactory = fn(number: u32) -> Vec<Vehicle>;
pub type RouteFactory = Box<dyn Fn(Vec<Stop>) -> Route>;
pub type GreedySolverFactory = Box<dyn Fn(u32) -> GreedySolver>;
pub type RouteServiceFactory = Box<dyn Fn(u32) -> RouteService>;

#[fixture]
pub fn distances() -> DistanceMatrix {
    HashMap::from([
        ((0, 1), 2.0),
        ((0, 2), 1.0),
        ((0, 3), 3.0),
        ((0, 4), 0.5),
        ((1, 0), 2.0),
        ((1, 2), 5.0),
        ((1, 3), 3.0),
        ((1, 4), 5.0),
        ((2, 0), 1.0),
        ((2, 1), 5.0),
        ((2, 3), 2.0),
        ((2, 4), 2.0),
        ((3, 0), 3.0),
        ((3, 1), 3.0),
        ((3, 2), 2.0),
        ((3, 4), 5.0),
        ((4, 0), 0.5),
        ((4, 1), 5.0),
        ((4, 2), 2.0),
        ((4, 3), 5.0),
    ])
}

#[fixture]
pub fn stops() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 0),
        Stop::new(1, 0),
        Stop::new(2, 0),
        Stop::new(3, 0),
        Stop::new(4, 100),
    ])
}

#[fixture]
pub fn full_stops() -> Vec<Stop> {
    Vec::from([Stop::new(0, 5), Stop::new(1, 100)])
}

#[fixture]
pub fn stops_with_crossings() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 10),
        Stop::new(3, 10),
        Stop::new(4, 10),
        Stop::new(1, 10),
        Stop::new(2, 10),
        Stop::new(0, 10),
    ])
}

#[fixture]
pub fn vehicle_factory() -> VehicleFactory {
    fn wrapper(number: u32) -> Vec<Vehicle> {
        let mut vehicles = Vec::new();

        for i in 0..number {
            vehicles.push(Vehicle::new(i, 10));
        }

        vehicles
    }

    wrapper
}

#[fixture]
pub fn distance_service(distances: DistanceMatrix, stops: Vec<Stop>) -> DistanceService {
    DistanceService::new(stops, &distances)
}

#[fixture]
pub fn two_opt(distances: DistanceMatrix, stops: Vec<Stop>) -> TwoOptSearcher {
    TwoOptSearcher::new(stops, &distances)
}

#[fixture]
pub(crate) fn stop_swapper(distances: DistanceMatrix, stops: Vec<Stop>) -> StopSwapper {
    StopSwapper::new(stops, &distances)
}

#[fixture]
pub fn route_factory(distance_service: DistanceService) -> RouteFactory {
    let wrapper = move |stops: Vec<Stop>| -> Route {
        let vehicle = Vehicle::new(0, 100);
        let mut route = Route::new(vehicle);

        route.add_stop(stops[0], Default::default()).unwrap();
        for (index, stop) in stops.iter().enumerate().skip(1) {
            route
                .add_stop(
                    *stop,
                    distance_service
                        .get_distance(&stops[index - 1], stop)
                        .unwrap(),
                )
                .unwrap();
        }

        route
    };

    Box::new(wrapper)
}

#[fixture]
pub fn greedy_solver_factory(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) -> GreedySolverFactory {
    let wrapper = move |number_of_vehicles: u32| -> GreedySolver {
        let vehicles = vehicle_factory(number_of_vehicles);

        GreedySolver::new(vehicles, &distances, stops.clone())
    };

    Box::new(wrapper)
}

#[fixture]
pub fn route_service_factory(
    stops: Vec<Stop>,
    distances: DistanceMatrix,
    vehicle_factory: VehicleFactory,
) -> RouteServiceFactory {
    let wrapper = move |number_of_vehicles| -> RouteService {
        let vehicles = vehicle_factory(number_of_vehicles);

        RouteService::new(vehicles, &distances, stops.clone())
    };

    Box::new(wrapper)
}
