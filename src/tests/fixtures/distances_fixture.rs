use std::collections::HashMap;

use rstest::fixture;

use crate::services::distance::distance_service::DistanceMatrix;

#[fixture]
pub fn distances() -> DistanceMatrix {
    HashMap::from([
        ((0, 1), 2.0),
        ((0, 2), 1.0),
        ((0, 3), 3.0),
        ((0, 4), 0.5),
        ((1, 0), 2.0),
        ((1, 2), 5.0),
        ((1, 3), 3.0),
        ((1, 4), 5.0),
        ((2, 0), 1.0),
        ((2, 1), 5.0),
        ((2, 3), 2.0),
        ((2, 4), 2.0),
        ((3, 0), 3.0),
        ((3, 1), 3.0),
        ((3, 2), 2.0),
        ((3, 4), 5.0),
        ((4, 0), 0.5),
        ((4, 1), 5.0),
        ((4, 2), 2.0),
        ((4, 3), 5.0),
    ])
}
