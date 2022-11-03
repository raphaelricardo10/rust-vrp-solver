use std::collections::HashMap;

use crate::domain::{
    route::{DistanceMatrix, Route},
    stop::Stop,
    vehicle::Vehicle,
};


pub type StopMap<'a> = HashMap<u32, &'a Stop>;
pub type RouteMap<'a> = HashMap<u32, Route<'a>>;

pub struct RouteService<'a> {
    routes: RouteMap<'a>,
    available_stops: StopMap<'a>,
}

impl<'a> RouteService<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> RouteService<'a> {
        let routes: RouteMap = RouteService::populate_routes(vehicles, distances);
        let available_stops: StopMap = RouteService::populate_available_stops(stops);

        RouteService { available_stops, routes }
    }

    pub fn populate_routes(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
    ) -> RouteMap {
        let mut routes: RouteMap = HashMap::new();
        for vehicle in vehicles {
            let id = vehicle.get_id();
            let route = Route::new(vehicle, distances);

            routes.insert(id, route);
        }

        routes
    }

    fn populate_available_stops(stops: &'a Vec<Stop>) -> StopMap{
        let mut available_stops: StopMap = HashMap::new();

        for stop in stops {
            available_stops.insert(stop.get_id(), stop);
        }

        available_stops
    }

    pub fn get_available_stops(&self) -> &StopMap {
        &self.available_stops
    }

    pub fn get_routes(&self) -> &RouteMap {
        &self.routes
    }

    pub fn get_vehicles(&self) -> Vec<&Vehicle> {
        self.routes.values().map(|x| x.get_vehicle()).collect()
    }

    pub fn assign_stop_to_route(&mut self, vehicle_id: u32, stop_id: u32) {
        let stop = self.available_stops.remove(&stop_id).unwrap();
        let vehicle = self.routes.get_mut(&vehicle_id).unwrap();

        vehicle.add_stop(stop).unwrap();
    }
}
