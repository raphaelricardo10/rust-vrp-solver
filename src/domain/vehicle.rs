use crate::errors::vehicle::{
    negative_capacity::NegativeVehicleCapacityError, vehicle_overload::VehicleOverloadError,
};

pub struct Vehicle {
    usage: u32,
    capacity: u32,
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
