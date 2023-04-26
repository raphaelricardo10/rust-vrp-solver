use crate::services::route::route_service::RouteMap;

use super::solution::Solution;

#[derive(Clone)]
pub struct VrpSolution {
    pub routes: RouteMap,
    pub total_distance: f32,
}

impl Default for VrpSolution {
    fn default() -> Self {
        Self {
            total_distance: f32::MAX,
            routes: Default::default(),
        }
    }
}

impl Solution for VrpSolution {
    type Cost = f32;
    type Data = RouteMap;

    fn get_cost(&self) -> Self::Cost {
        self.total_distance
    }

    fn get_data(&self) -> &Self::Data {
        &self.routes
    }
}

impl VrpSolution {
    pub fn new(routes: &RouteMap, total_distance: f32) -> Self {
        Self {
            total_distance,
            routes: routes.clone(),
        }
    }

    pub fn is_better_than(&self, other: &VrpSolution) -> bool {
        self.total_distance < other.total_distance
    }
}
