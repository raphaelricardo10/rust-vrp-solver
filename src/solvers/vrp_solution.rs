use crate::services::route::route_service::RouteMap;

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
