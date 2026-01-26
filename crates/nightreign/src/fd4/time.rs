#[repr(C)]
#[derive(Debug)]
pub struct FD4Time {
    vftable: usize,
    pub time: f32,
    _padc: u32,
}
