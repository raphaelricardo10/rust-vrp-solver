use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::neighbor::Neighbor;

#[derive(Copy, Clone)]
pub(crate) struct Neighborhood<'a> {
    pub(crate) previous: Neighbor<'a>,
    pub(crate) current: Neighbor<'a>,
    pub(crate) next: Neighbor<'a>,
    pub(crate) cost: f64,
}

impl<'a> Neighborhood<'a> {
    pub(crate) fn new(
        previous: Neighbor<'a>,
        current: Neighbor<'a>,
        next: Neighbor<'a>,
        distance_service: &DistanceService,
    ) -> Neighborhood<'a> {
        let mut neighborhood = Neighborhood {
            previous,
            current,
            next,
            cost: 0.0,
        };

        neighborhood.cost = neighborhood.calculate_cost(distance_service);

        neighborhood
    }

    pub(crate) fn from_stop_index(
        stops: &'a [Stop],
        stop_index: usize,
        distance_service: &DistanceService,
    ) -> Neighborhood<'a> {
        Self::new(
            Neighbor::new(stop_index - 1, &stops[stop_index - 1]),
            Neighbor::new(stop_index, &stops[stop_index]),
            Neighbor::new(stop_index + 1, &stops[stop_index + 1]),
            distance_service,
        )
    }

    fn calculate_cost(&self, distance_service: &DistanceService) -> f64 {
        distance_service.get_distance(self.previous.stop, self.current.stop)
            + distance_service.get_distance(self.current.stop, self.next.stop)
    }
}
