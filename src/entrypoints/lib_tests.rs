use crate::domain::{stop::Stop, vehicle::Vehicle};

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
pub unsafe extern "C" fn read_distance_matrix(distance_matrix: *mut f64, num_stops: usize, a: usize, b: usize) -> f64 {
    let arr = ndarray::aview_mut1(unsafe {
        std::slice::from_raw_parts_mut(distance_matrix, num_stops * num_stops)
    })
    .into_shape((num_stops, num_stops))
    .unwrap();

    arr[[a,b]]
}
