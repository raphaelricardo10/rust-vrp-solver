use std::{ptr, slice};

use crate::services::distance::distance_service::DistanceMatrix;

use crate::entrypoints::structures::distance_matrix::ABIDistanceMatrixEntry;

pub(super) unsafe fn distance_matrix_factory(
    ptr: *mut ABIDistanceMatrixEntry,
    len: usize,
) -> DistanceMatrix {
    std::slice::from_raw_parts(ptr, len)
        .iter()
        .map(|entry| ((entry.from, entry.to), entry.distance))
        .collect()
}

pub(super) unsafe fn vector_factory<T: Clone>(ptr: *mut T, len: usize) -> Vec<T> {
    slice::from_raw_parts(ptr, len).to_vec()
}

pub(super) unsafe fn copy_result<T>(src: Vec<T>, dest: *mut T) {
    ptr::copy_nonoverlapping(src.as_ptr(), dest, src.len());
}
