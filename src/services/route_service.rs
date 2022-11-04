use std::collections::HashMap;

use crate::{domain::{
    route::{DistanceMatrix, DistanceMatrixLine, Route},
    stop::Stop,
    vehicle::Vehicle,
}, errors::vehicle::vehicle_overload::VehicleOverloadError};

pub type StopMap<'a> = HashMap<u32, &'a Stop>;
pub type RouteMap<'a> = HashMap<u32, Route<'a>>;

pub struct RouteService<'a> {
    routes: RouteMap<'a>,
    available_stops: StopMap<'a>,
    distances: &'a DistanceMatrix,
}

impl<'a> RouteService<'a> {
    pub fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> RouteService<'a> {
        let routes: RouteMap = RouteService::populate_routes(vehicles, distances);
        let available_stops: StopMap = RouteService::populate_available_stops(stops);

        RouteService {
            available_stops,
            routes,
            distances,
        }
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

    fn populate_available_stops(stops: &'a Vec<Stop>) -> StopMap {
        let mut available_stops: StopMap = HashMap::new();

        for stop in stops {
            available_stops.insert(stop.get_id(), stop);
        }

        available_stops
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

    pub fn has_available_stop(&self) -> bool {
        self.available_stops.len() == 0
    }

    pub fn assign_stop_to_route(&mut self, vehicle_id: u32, stop_id: u32) -> Result<(), VehicleOverloadError>  {
        let stop = self.available_stops.remove(&stop_id).unwrap();
        let vehicle = self.routes.get_mut(&vehicle_id).unwrap();

        vehicle.add_stop(stop)
    }

    pub fn assign_starting_points(&mut self) -> Option<()> {
        let stop = self.available_stops.remove(&0)?;

        for (_, route) in &mut self.routes {
            route.add_stop(stop).ok();
        }

        Some(())
    }

    pub fn get_nearest_stop(&self, vehicle_id: u32) -> Option<&&Stop> {
        let current_stop_id = self.get_route(vehicle_id)?.get_current_stop()?.get_id();

        let ((_src_stop_id, dest_stop_id), _distance): DistanceMatrixLine = self
            .distances
            .iter()
            .filter(|x: &DistanceMatrixLine| self.available_stops.contains_key(&x.0 .1))
            .filter(|((src_stop_id, _dest_stop_id), _distance)| *src_stop_id == current_stop_id)
            .min_by(|x1: &DistanceMatrixLine, x2: &DistanceMatrixLine| {
                x1.1.partial_cmp(x2.1).unwrap()
            })?;

        self.available_stops.get(dest_stop_id)
    }
}
