// Source of name: RTTI
pub struct SprjScaleformValue {
    _vftable: usize,
    _unk08: u64,
    _unk10: u64,
    _unk18: usize,
    _unk20: u32,
    _unk28: u64,
    _unk30: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x38, size_of::<SprjScaleformValue>());
    }
}
