use crate::{services::route::route_service::RouteMap};

pub struct Solution {
    pub routes: RouteMap,
    pub total_distance: f64,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            total_distance: f64::MAX,
            routes: Default::default(),
        }
    }
}

impl Solution {
    pub fn new(routes: &RouteMap, total_distance: f64) -> Self {
        Self {
            total_distance,
            routes: routes.clone(),
        }
    }

    pub fn is_better_than(&self, other: &Solution) -> bool {
        self.total_distance < other.total_distance
    }
}
