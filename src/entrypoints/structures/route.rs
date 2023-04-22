#[repr(C)]
pub struct ABIRoute {
    pub(crate) vehicle_id: u32,
    pub(crate) stop_ids: *mut u32,
    pub(crate) number_of_stops: usize,
    pub(crate) total_distance: f32,
}
