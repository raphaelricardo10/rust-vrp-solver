use rstest::rstest;

use super::fixtures::services_fixture::{route_service_factory, RouteServiceFactory};

#[rstest]
fn route_service_started(route_service_factory: RouteServiceFactory) {
    let route_service = route_service_factory(2);

    assert_eq!(route_service.get_available_stops().len(), 5);
    assert_eq!(route_service.get_all_routes().len(), 2);
    assert_eq!(route_service.get_vehicles().len(), 2);
}

#[rstest]
fn can_assign_stop_to_route(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    route_service.assign_stop_to_route(0, 0).unwrap();

    assert_eq!(route_service.get_route(0).unwrap().stops.len(), 1)
}

#[rstest]
fn can_get_nearest_stop(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    route_service.assign_stop_to_route(0, 0).unwrap();

    assert_eq!(route_service.get_nearest_stop(0).unwrap().id, 2);
}

#[rstest]
fn can_get_k_nearest_stops(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    route_service.assign_stop_to_route(0, 0).unwrap();

    let k_nearest = route_service.get_k_nearest_stops(0, 3).unwrap();

    assert_eq!(k_nearest.len(), 3);

    assert_eq!(k_nearest[0].id, 2);
    assert_eq!(k_nearest[1].id, 1);
    assert_eq!(k_nearest[2].id, 3);
}

#[rstest]
fn cannot_get_infeasible_near_stops(route_service_factory: RouteServiceFactory) {
    let mut route_service = route_service_factory(1);

    route_service.assign_stop_to_route(0, 0).unwrap();

    let stop = route_service.get_nearest_stop(0).unwrap();

    assert_ne!(stop.id, 4)
}
