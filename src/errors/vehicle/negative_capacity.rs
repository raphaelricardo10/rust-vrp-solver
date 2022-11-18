use std::{fmt, error::Error};

#[derive(Debug)]
pub struct NegativeVehicleCapacityError {
    description: &'static str,
}

impl Default for NegativeVehicleCapacityError {
    fn default() -> Self {
        Self::new()
    }
}

impl NegativeVehicleCapacityError {
    pub fn new() -> NegativeVehicleCapacityError {
        NegativeVehicleCapacityError {
            description: "The capacity of vehicle cannot be lesser than zero",
        }
    }
}

impl fmt::Display for NegativeVehicleCapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for NegativeVehicleCapacityError {
    fn description(&self) -> &str {
        self.description
    }
}
