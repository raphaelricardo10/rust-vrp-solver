use crate::{
    domain::stop::Stop,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::neighborhood::Neighborhood;

pub(crate) struct StopSwapper {
    pub(crate) distance_service: DistanceService,
}

impl StopSwapper {
    pub fn new(stops: Vec<Stop>, distances: &DistanceMatrix) -> Self {
        Self {
            distance_service: DistanceService::new(stops, distances),
        }
    }

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
        neighborhood1: &'a Neighborhood<'a>,
        neighborhood2: &'a Neighborhood<'a>,
        distance_service: &DistanceService,
    ) -> (Neighborhood<'a>, Neighborhood<'a>) {
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
        mut neighborhood1: &'a Neighborhood<'a>,
        mut neighborhood2: &'a Neighborhood<'a>,
        distance_service: &DistanceService,
    ) -> (Neighborhood<'a>, Neighborhood<'a>) {
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
    ) -> f64 {
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

    pub(crate) fn get_minimum_swap_cost(
        &self,
        neighborhood: &Neighborhood,
        stops: &Vec<Stop>,
    ) -> (usize, f64) {
        stops
            .iter()
            .take(stops.len() - 1)
            .enumerate()
            .skip(neighborhood.next.index - 1)
            .map(|(stop_index, _)| {
                (
                    stop_index,
                    self.calculate_swap_cost(
                        neighborhood,
                        &Neighborhood::from((stops.as_slice(), stop_index, &self.distance_service)),
                    ),
                )
            })
            .min_by(|(_, cost1), (_, cost2)| {
                cost1.partial_cmp(cost2).unwrap_or_else(|| {
                    panic!("it should be possible to compare the swap costs {cost1} and {cost2}")
                })
            })
            .expect("the stops vector should not be empty")
    }
}
