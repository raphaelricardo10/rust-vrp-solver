#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArgSizes {
    pub vehicles: usize,
    pub stops: usize,
    pub distances: usize,
    pub result: usize,
}
