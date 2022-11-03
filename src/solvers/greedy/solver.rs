use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route_service::RouteService,
};

pub fn solve<'a>(vehicle: Vehicle, distances: &'a DistanceMatrix, stops: &'a Vec<Stop>) -> Vec<u32> {

    let vehicles = &mut vec![vehicle];
    let mut route_service = RouteService::new(vehicles, distances, stops);

    route_service.assign_stop_to_route(0, 0);

    while !route_service.has_available_stop() {
        let current_stop_pos = route_service.get_route(0).get_stops().len() - 1;
        let current_stop_id = route_service
            .get_route(0)
            .get_stops()
            .get(current_stop_pos)
            .unwrap()
            .get_id();
        let nearest_stop = route_service.get_nearest_stop(current_stop_id);
        route_service.assign_stop_to_route(0, nearest_stop.get_id());
    }

    route_service
        .get_route(0)
        .get_stops()
        .iter()
        .map(|x| x.get_id()).collect()
}