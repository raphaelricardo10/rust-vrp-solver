use crate::{
    domain::{stop::Stop, vehicle::Vehicle},
    services::distance::distance_service::DistanceMatrix,
};

use super::c_interfaces::c_distance_matrix::C_DistanceMatrixEntry;

#[no_mangle]
pub extern "C" fn update_vehicle(mut vehicle: Vehicle) -> Vehicle {
    vehicle.id = 2;
    vehicle.load(10).unwrap();

    vehicle
}

#[no_mangle]
pub extern "C" fn update_stop(mut stop: Stop) -> Stop {
    stop.id = 2;
    stop.usage = 10;

    stop
}

#[no_mangle]
pub unsafe extern "C" fn add_vehicle_to_array(
    vehicles_ptr: *mut Vehicle,
    num_vehicles: usize,
    result: *mut Vehicle,
) {
    let mut vehicles = unsafe { std::slice::from_raw_parts(vehicles_ptr, num_vehicles).to_vec() };

    vehicles.push(Vehicle::new(3, 130));
    std::ptr::copy_nonoverlapping(vehicles.as_ptr(), result, vehicles.len());
}

#[no_mangle]
pub unsafe extern "C" fn read_distance_matrix(
    distances_ptr: *mut C_DistanceMatrixEntry,
    num_entries: usize,
    a: u32,
    b: u32,
) -> f64 {
    let distance_matrix: DistanceMatrix = unsafe {
        std::slice::from_raw_parts(distances_ptr, num_entries)
            .iter()
            .map(|entry| ((entry.from, entry.to), entry.distance))
            .collect()
    };

    *distance_matrix.get(&(a, b)).unwrap()
}
