use crate::services::distance::distance_service::DistanceMatrix;
use crate::stop_swapper::StopSwapper;
use crate::tests::fixtures::distances_fixture::distances;
use crate::tests::fixtures::stops_fixture::stops;
use rstest::fixture;

use crate::{
    domain::stop::Stop, services::distance::distance_service::DistanceService,
    stop_swapper::path::Path,
};

use super::{services_fixture::distance_service, stops_fixture::stops_with_crossings};

pub struct PathFactory {
    pub stops: Vec<Stop>,
    pub distance_service: DistanceService,
}

impl<'a> PathFactory {
    fn new(stops: Vec<Stop>, distance_service: DistanceService) -> Self {
        Self {
            stops,
            distance_service,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn make_path(&'a self, stop_index: usize) -> Option<Path<'a>> {
        Path::from_stop_index(&self.stops, stop_index, &self.distance_service)
    }
}

#[fixture]
pub fn path_factory(
    stops_with_crossings: Vec<Stop>,
    distance_service: DistanceService,
) -> PathFactory {
    PathFactory::new(stops_with_crossings, distance_service)
}

#[fixture]
pub(crate) fn stop_swapper(distances: DistanceMatrix, stops: Vec<Stop>) -> StopSwapper {
    StopSwapper::new(stops, &distances)
}
