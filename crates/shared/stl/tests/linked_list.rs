mod common;

use crate::common::{DropCount, StdAlloc};
use fromsoftware_shared_stl::List;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn alloc() -> StdAlloc {
    StdAlloc::new(&COUNTER)
}
#[test]
fn list_new_is_empty() {
    let l: List<i32, _> = List::new_in(alloc());
    assert!(l.is_empty());
    assert_eq!(l.len(), 0);
}

#[test]
fn list_push_back_and_iter() {
    let mut l: List<i32, _> = List::new_in(alloc());
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    let collected: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected, [1, 2, 3]);
}

#[test]
fn list_push_front_and_iter() {
    let mut l: List<i32, _> = List::new_in(alloc());
    l.push_front(3);
    l.push_front(2);
    l.push_front(1);
    let collected: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected, [1, 2, 3]);
}

#[test]
fn list_push_front_and_back_mixed() {
    let mut l: List<i32, _> = List::new_in(alloc());
    l.push_back(2);
    l.push_front(1);
    l.push_back(3);
    let collected: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected, [1, 2, 3]);
}

#[test]
fn list_pop_front() {
    let mut l: List<i32, _> = List::new_in(alloc());
    l.push_back(10);
    l.push_back(20);
    l.push_back(30);
    assert_eq!(l.pop_front(), Some(10));
    assert_eq!(l.pop_front(), Some(20));
    assert_eq!(l.pop_front(), Some(30));
    assert_eq!(l.pop_front(), None);
    assert!(l.is_empty());
}

#[test]
fn list_pop_back() {
    let mut l: List<i32, _> = List::new_in(alloc());
    l.push_back(10);
    l.push_back(20);
    l.push_back(30);
    assert_eq!(l.pop_back(), Some(30));
    assert_eq!(l.pop_back(), Some(20));
    assert_eq!(l.pop_back(), Some(10));
    assert_eq!(l.pop_back(), None);
}

#[test]
fn list_pop_from_empty() {
    let mut l: List<i32, _> = List::new_in(alloc());
    assert_eq!(l.pop_front(), None);
    assert_eq!(l.pop_back(), None);
}

#[test]
fn list_iter_mut() {
    let mut l: List<i32, _> = List::new_in(alloc());
    for i in 0..5 {
        l.push_back(i);
    }
    for v in l.iter_mut() {
        *v *= 2;
    }
    let collected: Vec<_> = l.iter().copied().collect();
    assert_eq!(collected, [0, 2, 4, 6, 8]);
}

#[test]
fn list_len_tracks_pushes_and_pops() {
    let mut l: List<i32, _> = List::new_in(alloc());
    assert_eq!(l.len(), 0);
    l.push_back(1);
    assert_eq!(l.len(), 1);
    l.push_back(2);
    assert_eq!(l.len(), 2);
    l.push_front(0);
    assert_eq!(l.len(), 3);
    l.pop_front();
    assert_eq!(l.len(), 2);
    l.pop_back();
    assert_eq!(l.len(), 1);
}

#[test]
fn list_no_leak_empty() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _l: List<i32, _> = List::new_in(a.clone());
    }
    // sentinel node should be freed
    assert_eq!(a.live_count(), 0);
}

#[test]
fn list_no_leak_after_pushes() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut l: List<i32, _> = List::new_in(a.clone());
        for i in 0..64 {
            l.push_back(i);
        }
    }
    assert_eq!(a.live_count(), 0, "list leaked");
}

#[test]
fn list_no_leak_after_pop() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut l: List<i32, _> = List::new_in(a.clone());
        for i in 0..8 {
            l.push_back(i);
        }
        while l.pop_front().is_some() {}
    }
    assert_eq!(a.live_count(), 0);
}

#[test]
fn list_drop_calls_element_destructors() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut l: List<DropCount, _> = List::new_in(a.clone());
        for i in 0..16 {
            l.push_back(DropCount::new(&drop_count, i));
        }
        assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        16,
        "not all list elements dropped"
    );
    assert_eq!(a.live_count(), 0);
}

#[test]
fn list_pop_front_calls_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut l: List<DropCount, _> = List::new_in(a.clone());
    l.push_back(DropCount::new(&drop_count, 1));
    l.push_back(DropCount::new(&drop_count, 2));
    drop(l.pop_front());
    assert_eq!(drop_count.load(Ordering::Relaxed), 1);
    drop(l);
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn list_large_push_pop_order() {
    let mut l: List<i32, _> = List::new_in(alloc());
    for i in 0..256 {
        l.push_back(i);
    }
    let collected: Vec<_> = l.iter().copied().collect();
    let expected: Vec<_> = (0..256).collect();
    assert_eq!(collected, expected);
}

#[test]
fn list_used_as_stack_via_push_pop_front() {
    let mut l: List<i32, _> = List::new_in(alloc());
    for i in 0..8 {
        l.push_front(i);
    }
    let collected: Vec<_> = l.iter().copied().collect();
    let expected: Vec<_> = (0..8).rev().collect();
    assert_eq!(collected, expected);
}

#[test]
fn list_used_as_queue_via_push_back_pop_front() {
    let mut l: List<i32, _> = List::new_in(alloc());
    for i in 0..8 {
        l.push_back(i);
    }
    let mut out = Vec::new();
    while let Some(v) = l.pop_front() {
        out.push(v);
    }
    assert_eq!(out, (0..8).collect::<Vec<_>>());
}
