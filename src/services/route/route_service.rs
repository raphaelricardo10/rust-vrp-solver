use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, HashMap},
};

use crate::{
    domain::{
        route::{DistanceMatrix, Route},
        stop::Stop,
        vehicle::Vehicle,
    },
    errors::vehicle::vehicle_overload::VehicleOverloadError,
    services::distance::distance_service::DistanceService,
};

pub type StopMap<'a> = HashMap<u32, &'a Stop>;
pub type RouteMap<'a> = BTreeMap<u32, Route<'a>>;

pub struct RouteService<'a> {
    routes: RouteMap<'a>,
    available_stops: StopMap<'a>,
    distances: &'a DistanceMatrix,
    distance_service: DistanceService<'a>,
}

impl<'a> RouteService<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> RouteService<'a> {
        let routes: RouteMap = RouteService::map_routes(vehicles);
        let available_stops: StopMap = RouteService::map_stops(stops);

        RouteService {
            routes,
            distances,
            available_stops,
            distance_service: DistanceService::new(stops, distances),
        }
    }

    pub fn map_routes(vehicles: &'a mut Vec<Vehicle>) -> RouteMap {
        vehicles
            .iter_mut()
            .borrow_mut()
            .map(|vehicle| (vehicle.get_id(), Route::new(vehicle)))
            .collect()
    }

    fn map_stops(stops: &'a Vec<Stop>) -> StopMap {
        stops.iter().map(|stop| (stop.get_id(), stop)).collect()
    }

    pub fn get_available_stops(&self) -> &StopMap {
        &self.available_stops
    }

    pub fn get_route(&self, vehicle_id: u32) -> Option<&'a Route> {
        Some(self.routes.get(&vehicle_id)?)
    }

    pub fn get_all_routes(&self) -> &'a RouteMap {
        &self.routes
    }

    pub fn get_vehicles(&self) -> Vec<&Vehicle> {
        self.routes.values().map(|x| x.get_vehicle()).collect()
    }

    pub fn total_distance(&self) -> f64 {
        self.routes
            .iter()
            .map(|(_, route)| route.total_distance())
            .sum()
    }

    pub fn has_available_stop(&self) -> Option<bool> {
        for vehicle in self.get_vehicles().iter() {
            let route = self.get_route(vehicle.get_id())?;

            let feasible_stops: Vec<&Stop> = self
                .available_stops
                .values()
                .filter(|stop| self.is_stop_feasible(stop, route))
                .map(|stop| *stop)
                .collect();

            if feasible_stops.len() > 0 {
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
            Some(last_stop) => *self
                .distances
                .get(&(last_stop.get_id(), new_stop.get_id()))
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

    fn is_stop_feasible(&self, stop: &Stop, route: &Route) -> bool {
        if !self.available_stops.contains_key(&stop.get_id()) {
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
