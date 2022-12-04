#[repr(C)]
pub struct C_DistanceMatrixEntry{
    pub from: u32,
    pub to: u32,
    pub distance: f64
}