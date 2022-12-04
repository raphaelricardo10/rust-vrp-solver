use crate::services::distance::distance_service::DistanceMatrix;

use crate::entrypoints::c_interfaces::c_distance_matrix::CDistanceMatrixEntry;

pub unsafe fn distance_matrix(ptr: *mut CDistanceMatrixEntry, length: usize) -> DistanceMatrix {
    std::slice::from_raw_parts(ptr, length)
        .iter()
        .map(|entry| ((entry.from, entry.to), entry.distance))
        .collect()
}
