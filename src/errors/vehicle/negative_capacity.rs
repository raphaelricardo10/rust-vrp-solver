use std::{fmt, error::Error};

#[derive(Debug)]
pub struct NegativeVehicleCapacityError<'a> {
    description: &'a str,
}

impl<'a> NegativeVehicleCapacityError<'a> {
    pub fn new() -> NegativeVehicleCapacityError<'a> {
        NegativeVehicleCapacityError {
            description: "The capacity of vehicle cannot be lesser than zero",
        }
    }
}

impl<'a> fmt::Display for NegativeVehicleCapacityError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl<'a> Error for NegativeVehicleCapacityError<'a> {
    fn description(&self) -> &str {
        self.description
    }
}
