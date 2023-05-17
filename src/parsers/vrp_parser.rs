use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceMatrix,
};

pub struct VrpInputs {
    pub vehicles: Vec<Vehicle>,
    pub stops: Vec<Stop>,
    pub distances: DistanceMatrix,
}

pub trait VrpParser {
    fn parse(&self) -> VrpInputs;
}
