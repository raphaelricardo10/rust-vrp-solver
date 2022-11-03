use crate::domain::{
    route::{DistanceMatrix, Route},
    vehicle::Vehicle, stop::Stop,
};

pub struct RouteService<'a> {
    stops: Vec<Stop>,
    routes: Vec<Route<'a>>,
    vehicles: Vec<Vehicle>,
    distances: DistanceMatrix,
}

impl<'a> RouteService<'a> {
    pub fn new(&'a mut self, vehicles: Vec<Vehicle>, distances: DistanceMatrix, stops: Vec<Stop>) {
        self.stops = stops;
        self.vehicles = vehicles;
        self.distances = distances;

        self.routes = Vec::new();

        for vehicle in &mut self.vehicles {
            self.routes.push(Route::new(vehicle, &self.distances));
        }
    }

    pub fn get_stops(&self) -> &Vec<Stop> {
        &self.stops
    }

    pub fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }

    pub fn get_vehicles(&self) -> &Vec<Vehicle> {
        &self.vehicles
    }

}
