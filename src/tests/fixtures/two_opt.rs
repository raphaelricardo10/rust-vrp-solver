use std::rc::Rc;

use rstest::fixture;

use crate::{
    domain::stop::Stop,
    local_search::two_opt::TwoOptSearcher,
    services::distance::distance_service::{DistanceMatrix, DistanceService},
};

use super::{distances_fixture::distances, stops_fixture::stops};

#[fixture]
pub fn two_opt(distances: DistanceMatrix, stops: Vec<Stop>) -> TwoOptSearcher {
    let distance_service = Rc::new(DistanceService::new(stops, &distances));
    TwoOptSearcher::new(distance_service)
}
