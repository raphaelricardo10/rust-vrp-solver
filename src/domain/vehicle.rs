use crate::errors::vehicle::{
    negative_capacity::NegativeVehicleCapacityError, vehicle_overload::VehicleOverloadError,
};

#[derive(PartialEq, Eq, Hash)]
pub struct Vehicle {
    id: u32,
    usage: u32,
    capacity: u32,
}

impl Vehicle {
    pub fn new(id: u32, capacity: u32) -> Vehicle {
        Vehicle { id, capacity, usage: 0 }
    }

    pub fn get_id(&self) -> u32{
        self.id
    }

    pub fn get_capacity(&self) -> u32{
        self.capacity
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
