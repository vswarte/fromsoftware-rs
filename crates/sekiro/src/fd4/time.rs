#[repr(C)]
#[derive(Debug)]
pub struct FD4Time {
    vftable: usize,
    pub time: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x10, size_of::<FD4Time>());
    }
}
