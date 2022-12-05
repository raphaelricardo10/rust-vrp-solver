use crate::domain::{stop::Stop, vehicle::Vehicle};

use super::{
    c_interfaces::c_distance_matrix::CDistanceMatrixEntry,
    factories::{copy_result, distance_matrix_factory, vector_factory},
};

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
    let mut vehicles = vector_factory(vehicles_ptr, num_vehicles);

    vehicles.push(Vehicle::new(3, 130));

    copy_result(vehicles, result);
}

#[no_mangle]
pub unsafe extern "C" fn read_distance_matrix(
    distances_ptr: *mut CDistanceMatrixEntry,
    num_entries: usize,
    a: u32,
    b: u32,
) -> f64 {
    let distance_matrix = distance_matrix_factory(distances_ptr, num_entries);

    *distance_matrix.get(&(a, b)).unwrap()
}
