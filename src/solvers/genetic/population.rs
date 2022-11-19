use crate::services::route::route_service::RouteService;

use super::individual::Individual;

#[derive(Default)]
pub(crate) struct Population {
    pub(crate) individuals: Vec<Individual>,
}

impl Population {
    pub(super) fn new(individuals: Vec<Individual>) -> Self {
        Self { individuals }
    }
}
