use std::collections::{BTreeMap, HashMap};

use crate::{
    domain::{
        route::{DistanceMatrix, DistanceMatrixLine, Route},
        stop::Stop,
        vehicle::Vehicle,
    },
    errors::vehicle::vehicle_overload::VehicleOverloadError,
};

pub type StopMap<'a> = HashMap<u32, &'a Stop>;
pub type RouteMap<'a> = BTreeMap<u32, Route<'a>>;

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
        let routes: RouteMap = RouteService::populate_routes(vehicles);
        let available_stops: StopMap = RouteService::populate_available_stops(stops);

        RouteService {
            available_stops,
            routes,
            distances,
        }
    }

    pub fn populate_routes(
        vehicles: &'a mut Vec<Vehicle>,
    ) -> RouteMap {
        let mut routes: RouteMap = BTreeMap::new();
        for vehicle in vehicles {
            let id = vehicle.get_id();
            let route = Route::new(vehicle);

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

    fn can_add_stop(&self, stop_id: &u32, vehicle_id: &u32) -> Option<bool> {
        let stop = self.available_stops.get(&stop_id)?;

        Some(self.get_route(*vehicle_id)?.can_add_stop(stop))
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

    pub fn has_available_stop(&self) -> bool {
        for vehicle in self.get_vehicles().iter() {
            let feasible_stops = self
                .available_stops
                .iter()
                .filter(|x| self.can_add_stop(x.0, &vehicle.get_id()).unwrap());

            if feasible_stops.count() > 0 {
                return true;
            }
        }

        false
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

    fn get_feasible_stops(&self, vehicle_id: u32) -> impl Iterator<Item = DistanceMatrixLine> {
        let current_stop_id = self
            .get_route(vehicle_id)
            .unwrap()
            .get_current_stop()
            .unwrap()
            .get_id();

        self.distances
            .iter()
            .filter(|x: &DistanceMatrixLine| self.available_stops.contains_key(&x.0 .1))
            .filter(move |x: &DistanceMatrixLine| self.can_add_stop(&x.0 .1, &vehicle_id).unwrap())
            .filter(
                move |((src_stop_id, _dest_stop_id), _distance): &DistanceMatrixLine| {
                    *src_stop_id == current_stop_id
                },
            )
    }

    pub fn get_nearest_stop(&self, vehicle_id: u32) -> Option<&&Stop> {
        let ((_src_stop_id, dest_stop_id), _distance): DistanceMatrixLine =
            self.get_feasible_stops(vehicle_id).min_by(
                |x1: &DistanceMatrixLine, x2: &DistanceMatrixLine| x1.1.partial_cmp(x2.1).unwrap(),
            )?;

        self.available_stops.get(dest_stop_id)
    }

    pub fn get_k_nearest_stops(&self, vehicle_id: u32, k: usize) -> Vec<&&Stop> {
        let mut stops: Vec<DistanceMatrixLine> = self
            .get_feasible_stops(vehicle_id)
            .collect::<Vec<DistanceMatrixLine>>();

        stops.sort_by(|x1: &DistanceMatrixLine, x2: &DistanceMatrixLine| {
            x1.1.partial_cmp(x2.1).unwrap()
        });

        stops[0..k]
            .iter()
            .map(|x: &DistanceMatrixLine| self.available_stops.get(&x.0 .1).unwrap())
            .collect::<Vec<&&Stop>>()
    }
}
