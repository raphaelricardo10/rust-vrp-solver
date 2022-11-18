use crate::services::route::route_service::RouteService;

use super::individual::Individual;

pub(super) struct Population {
    individuals: Vec<Individual>,
}

impl Population {
    pub(super) fn new(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }
}
