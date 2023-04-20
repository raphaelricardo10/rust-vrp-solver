use crate::services::distance::distance_service::DistanceMatrix;
use crate::stop_swapper::StopSwapper;
use crate::tests::fixtures::distances_fixture::distances;
use crate::tests::fixtures::stops_fixture::stops;
use rstest::fixture;

use crate::{
    domain::stop::Stop, services::distance::distance_service::DistanceService,
    stop_swapper::neighborhood::Neighborhood,
};

use super::{services_fixture::distance_service, stops_fixture::stops_with_crossings};

pub struct NeighborhoodFactory {
    pub stops: Vec<Stop>,
    pub distance_service: DistanceService,
}

impl<'a> NeighborhoodFactory {
    fn new(stops: Vec<Stop>, distance_service: DistanceService) -> Self {
        Self {
            stops,
            distance_service,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn make_neighborhood(&'a self, stop_index: usize) -> Neighborhood<'a> {
        Neighborhood::from((self.stops.as_slice(), stop_index, &self.distance_service))
    }
}

#[fixture]
pub fn neighborhood_factory(
    stops_with_crossings: Vec<Stop>,
    distance_service: DistanceService,
) -> NeighborhoodFactory {
    NeighborhoodFactory::new(stops_with_crossings, distance_service)
}

#[fixture]
pub(crate) fn stop_swapper(distances: DistanceMatrix, stops: Vec<Stop>) -> StopSwapper {
    StopSwapper::new(stops, &distances)
}
