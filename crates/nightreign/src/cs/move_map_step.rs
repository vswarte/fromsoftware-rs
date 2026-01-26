#[repr(C)]
pub struct MoveMapStep {
    unk0: [u8; 0xF8],
    pub field_area: OwnedPtr<FieldArea>,
    unk100: [u8; 0x30],
    pub debug_pause: bool,
}
