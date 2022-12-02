use rstest::fixture;

use crate::{
    domain::stop::Stop, local_search::two_opt::TwoOptSearcher,
    services::distance::distance_service::DistanceMatrix,
};

use super::{distances_fixture::distances, stops_fixture::stops};

#[fixture]
pub fn two_opt(distances: DistanceMatrix, stops: Vec<Stop>) -> TwoOptSearcher {
    TwoOptSearcher::new(stops, &distances)
}
