use std::collections::HashMap;

use crate::services::route::route_service::RouteMap;

type Result = HashMap<u32, Vec<u32>>;

pub struct Solution {
    pub total_distance: f64,
    pub result: HashMap<u32, Vec<u32>>,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            result: Default::default(),
            total_distance: f64::MAX,
        }
    }
}

impl Solution {
    pub fn new(routes: &RouteMap, total_distance: f64) -> Self {
        Self {
            total_distance,
            result: Self::map_result(routes),
        }
    }

    fn map_result(routes: &RouteMap) -> Result {
        routes
            .iter()
            .map(|(vehicle_id, route)| -> (u32, Vec<u32>) {
                (
                    *vehicle_id,
                    route.stops.iter().map(|stop| stop.id).collect(),
                )
            })
            .collect()
    }

    pub fn is_better_than(&self, other: &Solution) -> bool {
        self.total_distance < other.total_distance
    }
}
