mod common;

use common::StdAlloc;
use fromsoftware_shared_stl::BasicString;
use std::sync::atomic::AtomicUsize;

fn std_alloc() -> StdAlloc {
    static C: AtomicUsize = AtomicUsize::new(0);
    StdAlloc::new(&C)
}
type NarrowString = BasicString<u8, StdAlloc>;
type WideString = BasicString<u16, StdAlloc>;
type U32String = BasicString<u32, StdAlloc>;

#[test]
fn narrow_empty() {
    let s = NarrowString::new_in(std_alloc());
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.as_code_units(), b"");
}

#[test]
fn narrow_sso_short_string() {
    let s = NarrowString::from_units_in(b"hello, world!!!", std_alloc());
    assert_eq!(s.len(), 15);
    assert_eq!(s.as_code_units(), b"hello, world!!!");
}

#[test]
fn narrow_sso_boundary_just_fits() {
    let data = b"123456789012345";
    assert_eq!(data.len(), 15);
    let s = NarrowString::from_units_in(data, std_alloc());
    assert_eq!(s.len(), 15);
    assert_eq!(s.as_code_units(), data);
    assert!(s.capacity() < 16, "should still be SSO");
}

#[test]
fn narrow_sso_boundary_just_over() {
    let data = b"1234567890123456";
    assert_eq!(data.len(), 16);
    let s = NarrowString::from_units_in(data, std_alloc());
    assert_eq!(s.len(), 16);
    assert_eq!(s.as_code_units(), data);
    assert_eq!(s.capacity(), 16, "heap capacity should equal len");
}

#[test]
fn narrow_heap_long_string() {
    let data: Vec<u8> = (0u8..=127).collect();
    let s = NarrowString::from_units_in(&data, std_alloc());
    assert_eq!(s.len(), 128);
    assert_eq!(s.as_code_units(), &data[..]);
}

#[test]
fn narrow_nul_terminator_sso() {
    let s = NarrowString::from_units_in(b"hi", std_alloc());
    // The byte after the data should be NUL in the SSO buffer
    let ptr = s.as_ptr();
    let after = unsafe { *ptr.add(s.len()) };
    assert_eq!(after, 0, "SSO string must be NUL-terminated");
}

#[test]
fn narrow_nul_terminator_heap() {
    let data: Vec<u8> = b"a".repeat(100);
    let s = NarrowString::from_units_in(&data, std_alloc());
    let ptr = s.as_ptr();
    let after = unsafe { *ptr.add(s.len()) };
    assert_eq!(after, 0, "heap string must be NUL-terminated");
}

#[test]
fn narrow_no_leak_sso() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    let before = a.live_count();
    {
        let _s = NarrowString::from_units_in(b"short", a.clone());
    }
    // SSO strings should not allocate
    assert_eq!(a.live_count(), before, "SSO must not allocate");
}

#[test]
fn narrow_no_leak_heap() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _s = NarrowString::from_units_in(b"x".repeat(100), a.clone());
        assert_eq!(a.live_count(), 1, "should have exactly one heap allocation");
    }
    assert_eq!(a.live_count(), 0, "heap string leaked");
}

#[test]
fn narrow_empty_new_in_no_alloc() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    let before = a.live_count();
    {
        let _s = NarrowString::new_in(a.clone());
    }
    assert_eq!(a.live_count(), before, "new_in must not allocate");
}

#[test]
fn narrow_as_ptr_sso_points_into_self() {
    let s = NarrowString::from_units_in(b"abc", std_alloc());
    let ptr = s.as_ptr() as usize;
    let self_start = &s as *const _ as usize;
    let self_end = self_start + std::mem::size_of_val(&s);
    assert!(
        ptr >= self_start && ptr < self_end,
        "SSO ptr should point inside the struct"
    );
}

#[test]
fn narrow_as_ptr_heap_points_outside_self() {
    let data = b"x".repeat(64);
    let s = NarrowString::from_units_in(&data, std_alloc());
    let ptr = s.as_ptr() as usize;
    let self_start = &s as *const _ as usize;
    let self_end = self_start + std::mem::size_of_val(&s);
    assert!(
        ptr < self_start || ptr >= self_end,
        "heap ptr should point outside the struct"
    );
}

#[test]
fn wide_empty() {
    let s = WideString::new_in(std_alloc());
    assert!(s.is_empty());
    assert_eq!(s.as_code_units(), &[] as &[u16]);
}

#[test]
fn wide_sso_fits() {
    let data: Vec<u16> = (b'a'..=b'g').map(|c| c as u16).collect();
    assert_eq!(data.len(), 7);
    let s = WideString::from_units_in(&data, std_alloc());
    assert_eq!(s.len(), 7);
    assert_eq!(s.as_code_units(), &data[..]);
    assert!(s.capacity() < 8);
}

#[test]
fn wide_sso_boundary_over() {
    let data: Vec<u16> = (0u16..8).collect();
    let s = WideString::from_units_in(&data, std_alloc());
    assert_eq!(s.len(), 8);
    assert_eq!(s.as_code_units(), &data[..]);
    assert_eq!(s.capacity(), 8);
}

#[test]
fn wide_nul_terminator_sso() {
    let data: Vec<u16> = vec![0x0068, 0x0069]; // "hi"
    let s = WideString::from_units_in(&data, std_alloc());
    let after = unsafe { *s.as_ptr().add(s.len()) };
    assert_eq!(after, 0u16, "wide SSO string must be NUL-terminated");
}

#[test]
fn wide_nul_terminator_heap() {
    let data: Vec<u16> = vec![0x0041u16; 32]; // 'A' x 32
    let s = WideString::from_units_in(&data, std_alloc());
    let after = unsafe { *s.as_ptr().add(s.len()) };
    assert_eq!(after, 0u16, "wide heap string must be NUL-terminated");
}

#[test]
fn wide_no_leak_heap() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let data: Vec<u16> = vec![0x0041u16; 32];
        let _s = WideString::from_units_in(&data, a.clone());
        assert_eq!(a.live_count(), 1);
    }
    assert_eq!(a.live_count(), 0, "wide heap string leaked");
}

#[test]
fn u32_sso_fits() {
    let data: Vec<u32> = vec![0x1F600, 0x1F601, 0x1F602]; // emoji codepoints
    let s = U32String::from_units_in(&data, std_alloc());
    assert_eq!(s.len(), 3);
    assert_eq!(s.as_code_units(), &data[..]);
    assert!(s.capacity() < 4);
}

#[test]
fn u32_heap() {
    let data: Vec<u32> = (0u32..32).collect();
    let s = U32String::from_units_in(&data, std_alloc());
    assert_eq!(s.len(), 32);
    assert_eq!(s.as_code_units(), &data[..]);
}

#[test]
fn u32_no_leak_heap() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let data: Vec<u32> = (0u32..32).collect();
        let _s = U32String::from_units_in(&data, a.clone());
    }
    assert_eq!(a.live_count(), 0, "u32 heap string leaked");
}

#[test]
fn narrow_from_empty_slice() {
    let s = NarrowString::from_units_in(b"", std_alloc());
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.as_code_units(), b"");
    // NUL terminator should still be present
    let after = unsafe { *s.as_ptr() };
    assert_eq!(after, 0);
}

#[test]
fn wide_from_empty_slice() {
    let s = WideString::from_units_in([], std_alloc());
    assert!(s.is_empty());
}

#[test]
fn string_heap_drop_frees_allocation() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    assert_eq!(a.live_count(), 0);
    {
        let _s = NarrowString::from_units_in(b"x".repeat(64), a.clone());
        assert_eq!(a.live_count(), 1, "should have one live allocation");
    }
    assert_eq!(a.live_count(), 0, "heap string not freed on drop");
}

#[test]
fn string_sso_drop_no_allocation() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _s = NarrowString::from_units_in(b"short", a.clone());
    }
    assert_eq!(a.live_count(), 0, "SSO string must never allocate");
}

#[test]
fn string_multiple_heap_drops() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _a = NarrowString::from_units_in(b"a".repeat(32), a.clone());
        let _b = NarrowString::from_units_in(b"b".repeat(64), a.clone());
        let _c = NarrowString::from_units_in(b"c".repeat(128), a.clone());
        assert_eq!(a.live_count(), 3);
    }
    assert_eq!(a.live_count(), 0, "not all heap strings were freed");
}

#[test]
fn string_drop_in_middle_of_scope() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    let s1 = NarrowString::from_units_in(b"a".repeat(32), a.clone());
    let s2 = NarrowString::from_units_in(b"b".repeat(32), a.clone());
    assert_eq!(a.live_count(), 2);
    drop(s1);
    assert_eq!(
        a.live_count(),
        1,
        "s1 allocation not freed after explicit drop"
    );
    drop(s2);
    assert_eq!(a.live_count(), 0);
}
