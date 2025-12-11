#[repr(C)]
pub struct ItemLotUtil {
    pub item_lot: u32,
    pub is_map_item_lot: bool,
    unk5: [u8; 3],
    unk8: u32,
    unkc: u32,
    unk10: u32,
    unk14: u32,
    unk18: usize,
}
