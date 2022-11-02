use std::collections::HashMap;
use super::{vehicle::Vehicle, stop::Stop};

pub struct Route {
    pub vehicle: Vehicle,
    pub stops: HashMap<u32, Stop>,
    pub distances: HashMap<u32, u32>,
}