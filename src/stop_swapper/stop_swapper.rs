use std::rc::Rc;

use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::neighborhood::Neighborhood;

pub(crate) struct StopSwapper {
    pub(crate) distance_service: Rc<DistanceService>,
}

impl StopSwapper {
    fn are_neighborhoods_consecutive(
        neighborhood1: &Neighborhood,
        neighborhood2: &Neighborhood,
    ) -> bool {
        if neighborhood1.next.index == neighborhood2.current.index {
            return true;
        }

        if neighborhood1.previous.index == neighborhood2.current.index {
            return true;
        }

        false
    }

    fn swap_non_consecutive_neighborhoods<'a>(
        neighborhood1: &'a Neighborhood,
        neighborhood2: &'a Neighborhood,
        distance_service: &DistanceService,
    ) -> (Neighborhood, Neighborhood) {
        let swapped_neighborhood_1 = Neighborhood::new(
            neighborhood1.previous,
            neighborhood2.current,
            neighborhood1.next,
            distance_service,
        );

        let swapped_neighborhood_2 = Neighborhood::new(
            neighborhood2.previous,
            neighborhood1.current,
            neighborhood2.next,
            distance_service,
        );

        (swapped_neighborhood_1, swapped_neighborhood_2)
    }

    fn swap_consecutive_neighborhoods<'a>(
        mut neighborhood1: &'a Neighborhood,
        mut neighborhood2: &'a Neighborhood,
        distance_service: &DistanceService,
    ) -> (Neighborhood, Neighborhood) {
        if neighborhood1.previous.stop.id == neighborhood2.current.stop.id {
            std::mem::swap(&mut neighborhood1, &mut neighborhood2);
        }

        let swapped_neighborhood_1 = Neighborhood::new(
            neighborhood1.previous,
            neighborhood2.current,
            neighborhood1.current,
            distance_service,
        );

        let swapped_neighborhood_2 = Neighborhood::new(
            neighborhood2.current,
            neighborhood1.current,
            neighborhood2.next,
            distance_service,
        );

        (swapped_neighborhood_1, swapped_neighborhood_2)
    }

    pub(crate) fn calculate_swap_cost(
        &self,
        neighborhood1: &Neighborhood,
        neighborhood2: &Neighborhood,
    ) -> f32 {
        let (swapped_neighborhood1, swapped_neighborhood2);

        if Self::are_neighborhoods_consecutive(neighborhood1, neighborhood2) {
            (swapped_neighborhood1, swapped_neighborhood2) = Self::swap_consecutive_neighborhoods(
                neighborhood1,
                neighborhood2,
                &self.distance_service,
            );
        } else {
            (swapped_neighborhood1, swapped_neighborhood2) =
                Self::swap_non_consecutive_neighborhoods(
                    neighborhood1,
                    neighborhood2,
                    &self.distance_service,
                );
        }

        (swapped_neighborhood1.cost + swapped_neighborhood2.cost)
            - (neighborhood1.cost + neighborhood2.cost)
    }
}
