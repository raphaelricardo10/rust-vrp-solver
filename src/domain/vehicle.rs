use std::{error::Error, fmt};

pub struct Vehicle {
    usage: u32,
    capacity: u32,
}

#[derive(Debug)]
pub struct VehicleOverloadError<'a> {
    description: &'a str,
}

#[derive(Debug)]
pub struct NegativeVehicleCapacityError<'a> {
    description: &'a str,
}

impl<'a> NegativeVehicleCapacityError<'a> {
    fn new() -> NegativeVehicleCapacityError<'a> {
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

impl<'a> VehicleOverloadError<'a> {
    fn new() -> VehicleOverloadError<'a> {
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

impl Vehicle {
    pub fn new(capacity: u32) -> Vehicle {
        Vehicle { capacity, usage: 0 }
    }

    pub fn get_capacity(&self) {
        self.capacity;
    }

    pub fn load(&mut self, quantity: u32) -> Result<(), VehicleOverloadError> {
        if self.usage + quantity > self.capacity {
            return Err(VehicleOverloadError::new());
        }

        self.usage += quantity;
        Ok(())
    }

    pub fn unload(&mut self, quantity: u32) -> Result<(), NegativeVehicleCapacityError> {
        if quantity > self.usage {
            return Err(NegativeVehicleCapacityError::new());
        }

        self.usage = quantity;
        Ok(())
    }
}
