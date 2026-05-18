mod common;

use crate::common::StdAlloc;
use fromsoftware_shared_stl::VectorBool;
use std::sync::atomic::AtomicUsize;

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn alloc() -> StdAlloc {
    StdAlloc::new(&COUNTER)
}

#[test]
fn vector_bool_new_is_empty() {
    let v: VectorBool<_> = VectorBool::new_in(alloc());
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 0);
}

#[test]
fn vector_bool_push_and_get() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    v.push_back(true);
    v.push_back(false);
    v.push_back(true);
    assert_eq!(v.get(0), Some(true));
    assert_eq!(v.get(1), Some(false));
    assert_eq!(v.get(2), Some(true));
    assert_eq!(v.get(3), None);
}

#[test]
fn vector_bool_pop_back() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    v.push_back(true);
    v.push_back(false);
    assert_eq!(v.pop_back(), Some(false));
    assert_eq!(v.pop_back(), Some(true));
    assert_eq!(v.pop_back(), None);
}

#[test]
fn vector_bool_set() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    v.push_back(false);
    v.push_back(false);
    v.set(0, true);
    assert_eq!(v.get(0), Some(true));
    assert_eq!(v.get(1), Some(false));
}

#[test]
fn vector_bool_flip() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for b in [true, false, true, false] {
        v.push_back(b);
    }
    v.flip();
    let bits: Vec<_> = v.iter().collect();
    assert_eq!(bits, [false, true, false, true]);
}

#[test]
fn vector_bool_count_ones_zeros() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for b in [true, false, true, true, false] {
        v.push_back(b);
    }
    assert_eq!(v.count_ones(), 3);
    assert_eq!(v.count_zeros(), 2);
}

#[test]
fn vector_bool_any_all() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    v.push_back(false);
    v.push_back(false);
    assert!(!v.any());
    assert!(!v.all());

    v.push_back(true);
    assert!(v.any());
    assert!(!v.all());

    let mut all_true: VectorBool<_> = VectorBool::new_in(alloc());
    all_true.push_back(true);
    all_true.push_back(true);
    assert!(all_true.all());
}

#[test]
fn vector_bool_iter() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    let bits = [true, false, true, true, false, true];
    for &b in &bits {
        v.push_back(b);
    }
    let collected: Vec<_> = v.iter().collect();
    assert_eq!(collected, bits);
}

#[test]
fn vector_bool_into_iter() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for b in [true, false, true] {
        v.push_back(b);
    }
    let collected: Vec<_> = (&v).into_iter().collect();
    assert_eq!(collected, [true, false, true]);
}

#[test]
fn vector_bool_exact_size_iterator() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for _ in 0..7 {
        v.push_back(true);
    }
    let mut it = v.iter();
    assert_eq!(it.len(), 7);
    it.next();
    assert_eq!(it.len(), 6);
}

#[test]
fn vector_bool_spans_multiple_words() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for i in 0..65 {
        v.push_back(i % 3 == 0);
    }
    assert_eq!(v.len(), 65);
    for i in 0..65 {
        assert_eq!(v.get(i), Some(i % 3 == 0), "mismatch at index {i}");
    }
}

#[test]
fn vector_bool_no_leak_empty() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _v: VectorBool<_> = VectorBool::new_in(a.clone());
    }
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_bool_no_leak_after_growth() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut v: VectorBool<_> = VectorBool::new_in(a.clone());
        for i in 0..256 {
            v.push_back(i % 2 == 0);
        }
    }
    assert_eq!(a.live_count(), 0, "vector<bool> leaked");
}

#[test]
fn vector_bool_clear_on_empty_is_noop() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    v.clear();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 0);
}

#[test]
fn vector_bool_clear_makes_vector_empty() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for b in [true, false, true, true, false] {
        v.push_back(b);
    }
    v.clear();
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
    assert_eq!(v.iter().count(), 0);
}

#[test]
fn vector_bool_clear_retains_capacity() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for _ in 0..64 {
        v.push_back(true);
    }
    let cap_before = v.capacity();
    v.clear();
    assert_eq!(
        v.capacity(),
        cap_before,
        "clear must not release the underlying word buffer"
    );
}

#[test]
fn vector_bool_clear_zeroes_all_bits() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for _ in 0..64 {
        v.push_back(true);
    }
    v.clear();
    for _ in 0..32 {
        v.push_back(false);
    }
    assert_eq!(v.count_ones(), 0, "stale set bits must not survive clear");
    assert_eq!(v.count_zeros(), 32);
}

#[test]
fn vector_bool_clear_keeps_buffer_allocation() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    let mut v: VectorBool<_> = VectorBool::new_in(a.clone());
    for _ in 0..64 {
        v.push_back(true);
    }
    assert_eq!(a.live_count(), 1);
    v.clear();
    assert_eq!(a.live_count(), 1, "buffer must still be live after clear");
    drop(v);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_bool_clear_then_reuse() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for _ in 0..16 {
        v.push_back(true);
    }
    v.clear();
    let bits = [false, true, false, true];
    for &b in &bits {
        v.push_back(b);
    }
    let collected: Vec<_> = v.iter().collect();
    assert_eq!(collected, bits);
}

#[test]
fn vector_bool_double_clear_is_safe() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for _ in 0..8 {
        v.push_back(true);
    }
    v.clear();
    v.clear();
    assert!(v.is_empty());
}

#[test]
fn vector_bool_clear_no_leak() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut v: VectorBool<_> = VectorBool::new_in(a.clone());
        for i in 0..256 {
            v.push_back(i % 2 == 0);
        }
        v.clear();
    }
    assert_eq!(
        a.live_count(),
        0,
        "clear + drop must leave no live allocations"
    );
}

#[test]
fn vector_bool_clear_across_word_boundary() {
    let mut v: VectorBool<_> = VectorBool::new_in(alloc());
    for i in 0..130 {
        v.push_back(i % 2 == 0);
    }
    assert_eq!(v.len(), 130);
    v.clear();
    assert_eq!(v.len(), 0);
    assert!(v.is_empty());
    v.push_back(true);
    assert_eq!(v.get(0), Some(true));
    assert_eq!(v.count_ones(), 1);
}
