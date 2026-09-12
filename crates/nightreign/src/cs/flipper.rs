#[repr(C)]
#[shared::singleton("CSFlipper")]
pub struct CSFlipperImp {
    unk0: [u8; 0x2D4],
    pub time_multiplier: f32,
}
