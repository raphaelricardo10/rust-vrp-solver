use rstest::fixture;

use crate::domain::stop::Stop;

#[fixture]
pub fn stops() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 0),
        Stop::new(1, 0),
        Stop::new(2, 0),
        Stop::new(3, 0),
        Stop::new(4, 100),
    ])
}

#[fixture]
pub fn full_stops() -> Vec<Stop> {
    Vec::from([Stop::new(0, 5), Stop::new(1, 100)])
}

#[fixture]
pub fn stops_with_crossings() -> Vec<Stop> {
    Vec::from([
        Stop::new(0, 10),
        Stop::new(3, 10),
        Stop::new(4, 10),
        Stop::new(1, 10),
        Stop::new(2, 10),
        Stop::new(0, 10),
    ])
}
