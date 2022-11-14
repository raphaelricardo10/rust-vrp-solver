use std::collections::{BTreeMap, HashMap};

use crate::{
    domain::{route::Route, stop::Stop, vehicle::Vehicle},
    errors::vehicle::vehicle_overload::VehicleOverloadError,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

pub type StopMap = HashMap<u32, Stop>;
pub type RouteMap = BTreeMap<u32, Route>;

pub struct RouteService {
    routes: RouteMap,
    all_stops: Vec<Stop>,
    available_stops: StopMap,
    distance_service: DistanceService,
}

impl RouteService {
    pub fn new(
        vehicles: Vec<Vehicle>,
        distances: &DistanceMatrix,
        stops: Vec<Stop>,
    ) -> RouteService {
        RouteService {
            all_stops: stops.clone(),
            routes: Self::map_routes(vehicles),
            available_stops: Self::map_stops(stops.clone()),
            distance_service: DistanceService::new(stops, distances),
        }
    }

    pub fn reset(&mut self) {
        for route in self.routes.values_mut() {
            route.reset();
        }

        self.available_stops = Self::map_stops(self.all_stops.clone());
    }

    pub fn map_routes(vehicles: Vec<Vehicle>) -> RouteMap {
        let mut route_map = BTreeMap::new();

        for vehicle in vehicles {
            route_map.insert(vehicle.id, Route::new(vehicle));
        }

        route_map
    }

    fn map_stops(stops: Vec<Stop>) -> StopMap {
        stops.iter().map(|stop| (stop.id, *stop)).collect()
    }

    pub fn get_available_stops(&self) -> &StopMap {
        &self.available_stops
    }

    pub fn get_route(&self, vehicle_id: u32) -> Option<&Route> {
        Some(self.routes.get(&vehicle_id)?)
    }

    pub fn get_route_mut(&mut self, vehicle_id: u32) -> Option<&mut Route> {
        Some(self.routes.get_mut(&vehicle_id)?)
    }

    pub fn get_all_routes(&self) -> &RouteMap {
        &self.routes
    }

    pub fn get_vehicles(&self) -> Vec<&Vehicle> {
        self.routes.values().map(|x| &x.vehicle).collect()
    }

    pub fn total_distance(&self) -> f64 {
        self.routes
            .iter()
            .map(|(_, route)| route.total_distance())
            .sum()
    }

    pub fn has_available_stop(&self) -> Option<bool> {
        for route in self.routes.values() {
            let feasible_stops_number = self
                .available_stops
                .values()
                .filter(|stop| self.is_stop_feasible(stop, route))
                .map(|stop| *stop)
                .count();

            if feasible_stops_number > 0 {
                return Some(true);
            }
        }

        Some(false)
    }

    pub fn assign_stop_to_route(
        &mut self,
        vehicle_id: u32,
        stop_id: u32,
    ) -> Result<(), VehicleOverloadError> {
        let route = self.routes.get_mut(&vehicle_id).unwrap();

        let new_stop = self.available_stops.remove(&stop_id).unwrap();

        let distance = match route.get_current_stop() {
            Some(last_stop) => self
                .distance_service
                .get_distance(&last_stop, &new_stop)
                .unwrap(),
            None => 0.0,
        };

        route.add_stop(new_stop, distance)
    }

    pub fn assign_starting_points(&mut self) -> Option<()> {
        let starting_stop = self.available_stops.remove(&0)?;

        for (_, route) in &mut self.routes {
            route.add_stop(starting_stop, 0.0).ok();
        }

        Some(())
    }

    pub fn assign_stop_points(&mut self) -> Option<()> {
        for (_, route) in &mut self.routes {
            let first_stop = route.stops.first()?;
            let last_stop = route.stops.last()?;
            let distance = self.distance_service.get_distance(last_stop, first_stop)?;

            route.add_stop(*first_stop, distance).ok();
        }

        Some(())
    }

    fn is_stop_feasible(&self, stop: &Stop, route: &Route) -> bool {
        if !self.available_stops.contains_key(&stop.id) {
            return false;
        }

        if !route.can_add_stop(stop) {
            return false;
        }

        true
    }

    pub fn get_nearest_stop(&self, vehicle_id: u32) -> Option<&Stop> {
        let route = self.get_route(vehicle_id)?;
        let current_stop = route.get_current_stop()?;

        self.distance_service
            .get_nearest_stop(current_stop, |stop| self.is_stop_feasible(stop, route))
    }

    pub fn get_k_nearest_stops(&self, vehicle_id: u32, k: usize) -> Option<Vec<&Stop>> {
        let route = self.get_route(vehicle_id)?;
        let current_stop = route.get_current_stop()?;

        Some(
            self.distance_service
                .get_k_nearest_stops(current_stop, k, |stop| self.is_stop_feasible(stop, route)),
        )
    }
}
