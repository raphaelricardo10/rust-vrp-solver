use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::neighbor::Neighbor;

#[derive(Copy, Clone)]
pub(crate) struct Neighborhood {
    pub(crate) previous: Neighbor,
    pub(crate) current: Neighbor,
    pub(crate) next: Neighbor,
    pub(crate) cost: f32,
}

pub(crate) type StopReference<'a, 'b> = (&'a [Stop], usize, &'b DistanceService);

impl<'a, 'b> From<StopReference<'a, 'b>> for Neighborhood {
    fn from((stops, stop_index, distance_service): StopReference<'a, 'b>) -> Self {
        Self::new(
            Neighbor {
                index: stop_index - 1,
                stop: stops[stop_index - 1],
            },
            Neighbor {
                index: stop_index,
                stop: stops[stop_index],
            },
            Neighbor {
                index: stop_index + 1,
                stop: stops[stop_index + 1],
            },
            distance_service,
        )
    }
}

impl Neighborhood {
    pub(crate) fn new(
        previous: Neighbor,
        current: Neighbor,
        next: Neighbor,
        distance_service: &DistanceService,
    ) -> Neighborhood {
        let mut neighborhood = Neighborhood {
            previous,
            current,
            next,
            cost: 0.0,
        };

        neighborhood.cost = neighborhood.calculate_cost(distance_service);

        neighborhood
    }

    fn calculate_cost(&self, distance_service: &DistanceService) -> f32 {
        distance_service.get_distance(&self.previous.stop, &self.current.stop)
            + distance_service.get_distance(&self.current.stop, &self.next.stop)
    }
}
