use crate::errors::vehicle::{
    negative_capacity::NegativeVehicleCapacityError, vehicle_overload::VehicleOverloadError,
};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Vehicle {
    usage: u32,
    pub id: u32,
    pub capacity: u32,
}

impl Vehicle {
    pub fn new(id: u32, capacity: u32) -> Vehicle {
        Vehicle {
            id,
            capacity,
            usage: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.usage = Default::default();
    }

    pub fn can_support_load(&self, quantity: u32) -> bool {
        self.usage + quantity < self.capacity
    }

    pub fn load(&mut self, quantity: u32) -> Result<(), VehicleOverloadError> {
        if !self.can_support_load(quantity) {
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
