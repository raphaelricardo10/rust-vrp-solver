#[repr(C)]
pub struct CDistanceMatrixEntry{
    pub from: u32,
    pub to: u32,
    pub distance: f64
}
