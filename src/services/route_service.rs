use crate::domain::{route::Route, vehicle::Vehicle};

pub struct RouteService {
    routes: Vec<Route>,
}

impl RouteService {
    pub fn init_routes(&mut self, vehicles: Vec<Vehicle>) {
        for vehicle in vehicles {

            let route = Route::new(vehicle);
            self.routes.push(route);
        }
    }
}
