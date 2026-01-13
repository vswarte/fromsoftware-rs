use std::ptr::NonNull;

#[repr(C)]
/// Source of name: RTTI
pub struct CSAiFunc {
    vftable: isize,
    pub ai_ins: NonNull<AiIns>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct AiIns {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSAiFunc>(), 0x10);
    }
}
