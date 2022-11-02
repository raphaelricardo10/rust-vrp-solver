use std::{fmt, error::Error};

#[derive(Debug)]
pub struct VehicleOverloadError<'a> {
    description: &'a str,
}

impl<'a> VehicleOverloadError<'a> {
    pub fn new() -> VehicleOverloadError<'a> {
        VehicleOverloadError {
            description: "The capacity of vehicle was overloaded",
        }
    }
}

impl<'a> fmt::Display for VehicleOverloadError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl<'a> Error for VehicleOverloadError<'a> {
    fn description(&self) -> &str {
        self.description
    }
}
