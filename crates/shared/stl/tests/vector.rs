mod common;

use crate::common::{DropCount, StdAlloc};
use fromsoftware_shared_stl::Vector;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn alloc() -> StdAlloc {
    StdAlloc::new(&COUNTER)
}

#[test]
fn vector_new_is_empty() {
    let v: Vector<i32, _> = Vector::new_in(alloc());
    assert!(v.is_empty());
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 0);
}

#[test]
fn vector_push_back_and_index() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    v.push_back(10);
    v.push_back(20);
    v.push_back(30);
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 10);
    assert_eq!(v[1], 20);
    assert_eq!(v[2], 30);
}

#[test]
fn vector_pop_back() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    v.push_back(1);
    v.push_back(2);
    assert_eq!(v.pop_back(), Some(2));
    assert_eq!(v.pop_back(), Some(1));
    assert_eq!(v.pop_back(), None);
    assert!(v.is_empty());
}

#[test]
fn vector_from_slice() {
    let v = Vector::from_slice_in(&[1i32, 2, 3, 4, 5], alloc());
    assert_eq!(v.len(), 5);
    assert_eq!(&*v, &[1, 2, 3, 4, 5]);
}

#[test]
fn vector_from_empty_slice() {
    let v = Vector::<i32, _>::from_slice_in(&[], alloc());
    assert!(v.is_empty());
    assert_eq!(v.capacity(), 0);
}

#[test]
fn vector_grows_beyond_initial_capacity() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..64 {
        v.push_back(i);
    }
    assert_eq!(v.len(), 64);
    let collected: Vec<_> = v.iter().copied().collect();
    let expected: Vec<_> = (0..64).collect();
    assert_eq!(collected, expected);
}

#[test]
fn vector_capacity_grows_at_least_1_5x() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    v.push_back(1); // triggers first grow, cap becomes 4
    assert!(v.capacity() == 4, "capacity did not grow to 4");
    let cap_after_first = v.capacity();
    // Fill to capacity to force next grow
    while v.len() < cap_after_first {
        v.push_back(0);
    }
    let cap_before = v.capacity();
    v.push_back(99);
    assert!(
        v.capacity() >= cap_before + cap_before / 2,
        "capacity did not grow by at least 1.5x"
    );
}

#[test]
fn vector_deref_slice() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..5 {
        v.push_back(i);
    }
    let slice: &[i32] = &v;
    assert_eq!(slice, &[0, 1, 2, 3, 4]);
}

#[test]
fn vector_deref_mut_slice() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..3 {
        v.push_back(i);
    }
    v[1] = 99;
    assert_eq!(&*v, &[0, 99, 2]);
}

#[test]
fn vector_iter() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..5 {
        v.push_back(i * 10);
    }
    let collected: Vec<_> = v.iter().copied().collect();
    assert_eq!(collected, [0, 10, 20, 30, 40]);
}

#[test]
fn vector_no_leak_empty() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _v: Vector<i32, _> = Vector::new_in(a.clone());
    }
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_no_leak_after_growth() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut v: Vector<i32, _> = Vector::new_in(a.clone());
        for i in 0..128 {
            v.push_back(i);
        }
    }
    assert_eq!(a.live_count(), 0, "vector leaked after growth");
}

#[test]
fn vector_drop_calls_element_destructors() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut v: Vector<DropCount, _> = Vector::new_in(a.clone());
        for i in 0..16 {
            v.push_back(DropCount::new(&drop_count, i));
        }
        assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        16,
        "not all elements dropped"
    );
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_pop_back_calls_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut v: Vector<DropCount, _> = Vector::new_in(a.clone());
    v.push_back(DropCount::new(&drop_count, 1));
    v.push_back(DropCount::new(&drop_count, 2));
    drop(v.pop_back());
    assert_eq!(drop_count.load(Ordering::Relaxed), 1);
    drop(v);
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_push_front_ordering() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(0);
    assert_eq!(&*v, &[0, 1, 2, 3]);
}

#[test]
fn vector_push_front_triggers_grow() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..4 {
        v.push_back(i);
    }
    assert_eq!(v.capacity(), 4);
    v.push_front(99);
    assert_eq!(v[0], 99);
    assert_eq!(&v[1..], &[0, 1, 2, 3]);
}

#[test]
fn vector_push_front_calls_destructor_on_drop() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut v: Vector<DropCount, _> = Vector::new_in(a.clone());
        for i in 0..8 {
            v.push_front(DropCount::new(&drop_count, i));
        }
    }
    assert_eq!(drop_count.load(Ordering::Relaxed), 8);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn vector_pop_front() {
    let mut v: Vector<i32, _> = Vector::new_in(alloc());
    for i in 0..4 {
        v.push_back(i);
    }
    assert_eq!(v.pop_front(), Some(0));
    assert_eq!(&*v, &[1, 2, 3]);
    assert_eq!(v.pop_front(), Some(1));
    assert_eq!(v.pop_front(), Some(2));
    assert_eq!(v.pop_front(), Some(3));
    assert_eq!(v.pop_front(), None);
}

#[test]
fn vector_pop_front_no_double_drop() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut v: Vector<DropCount, _> = Vector::new_in(a.clone());
        for i in 0..4 {
            v.push_back(DropCount::new(&drop_count, i));
        }
        while v.pop_front().is_some() {}
        assert_eq!(drop_count.load(Ordering::Relaxed), 4);
    }
    assert_eq!(drop_count.load(Ordering::Relaxed), 4);
    assert_eq!(a.live_count(), 0);
}
