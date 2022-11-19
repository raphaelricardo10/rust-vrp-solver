use crate::services::route::route_service::RouteService;

use super::individual::Individual;

#[derive(Default)]
pub(super) struct Population {
    pub(super) individuals: Vec<Individual>,
}

impl Population {
    pub(super) fn new(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }
}
