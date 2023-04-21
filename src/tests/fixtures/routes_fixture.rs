use super::services_fixture::distance_service;
use rstest::fixture;

use crate::{
    domain::{route::Route, stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceService,
};

pub type RouteFactory = Box<dyn Fn(Vec<Stop>) -> Route>;

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
                    distance_service.get_distance(&stops[index - 1], stop),
                )
                .unwrap();
        }

        route
    };

    Box::new(wrapper)
}
