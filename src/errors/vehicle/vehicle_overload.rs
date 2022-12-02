use std::{error::Error, fmt};

#[derive(Debug)]
pub struct VehicleOverloadError {
    description: &'static str,
}

impl Default for VehicleOverloadError {
    fn default() -> Self {
        Self::new()
    }
}

impl VehicleOverloadError {
    pub fn new() -> VehicleOverloadError {
        VehicleOverloadError {
            description: "The capacity of vehicle was overloaded",
        }
    }
}

impl fmt::Display for VehicleOverloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for VehicleOverloadError {
    fn description(&self) -> &str {
        self.description
    }
}
