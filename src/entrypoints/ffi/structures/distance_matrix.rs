#[repr(C)]
pub struct FFIDistanceMatrixEntry {
    pub from: u32,
    pub to: u32,
    pub distance: f32,
}
