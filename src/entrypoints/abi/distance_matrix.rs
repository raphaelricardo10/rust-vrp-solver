#[repr(C)]
pub struct ABIDistanceMatrixEntry{
    pub from: u32,
    pub to: u32,
    pub distance: f64
}
