use crate::domain::{
    route::{DistanceMatrix, Route},
    stop::Stop,
    vehicle::Vehicle,
};

pub struct RouteService<'a> {
    routes: Vec<Route<'a>>,
    available_stops: &'a Vec<Stop>,
}

impl<'a> RouteService<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        available_stops: &'a Vec<Stop>,
    ) -> RouteService<'a> {
        let routes = RouteService::populate_routes(vehicles, distances);

        RouteService { available_stops, routes }
    }

    pub fn populate_routes(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
    ) -> Vec<Route> {
        let mut routes: Vec<Route> = Vec::new();
        for vehicle in vehicles {
            routes.push(Route::new(vehicle, distances));
        }

        routes
    }

    pub fn get_stops(&self) -> &Vec<Stop> {
        &self.available_stops
    }

    pub fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }

    pub fn get_vehicles(&self) -> Vec<&Vehicle> {
        self.routes.iter().map(|x| x.get_vehicle()).collect()
    }
}
