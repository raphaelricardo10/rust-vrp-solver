use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::path_node::Neighbor;

#[derive(Copy, Clone)]
pub(crate) struct Neighborhood<'a> {
    pub(crate) prev: Neighbor<'a>,
    pub(crate) current: Neighbor<'a>,
    pub(crate) next: Neighbor<'a>,
    pub(crate) cost: f64,
}

impl<'a> Neighborhood<'a> {
    pub(crate) fn new(
        prev: Neighbor<'a>,
        current: Neighbor<'a>,
        next: Neighbor<'a>,
        distance_service: &DistanceService,
    ) -> Option<Neighborhood<'a>> {
        let mut path = Neighborhood {
            prev,
            current,
            next,
            cost: 0.0,
        };

        path.cost = path.calculate_cost(distance_service)?;

        Some(path)
    }

    pub(crate) fn from_stop_index(
        stops: &'a [Stop],
        stop_index: usize,
        distance_service: &DistanceService,
    ) -> Option<Neighborhood<'a>> {
        Self::new(
            Neighbor::new(stop_index - 1, &stops[stop_index - 1]),
            Neighbor::new(stop_index, &stops[stop_index]),
            Neighbor::new(stop_index + 1, &stops[stop_index + 1]),
            distance_service,
        )
    }

    fn calculate_cost(&self, distance_service: &DistanceService) -> Option<f64> {
        Some(
            distance_service.get_distance(self.prev.stop, self.current.stop)?
                + distance_service.get_distance(self.current.stop, self.next.stop)?,
        )
    }
}
