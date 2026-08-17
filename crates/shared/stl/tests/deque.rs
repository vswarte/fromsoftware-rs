mod common;

use crate::common::{DropCount, StdAlloc};
use fromsoftware_shared_stl::Deque;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn alloc() -> StdAlloc {
    StdAlloc::new(&COUNTER)
}

#[test]
fn deque_new_is_empty() {
    let d: Deque<i32, _> = Deque::new_in(alloc());
    assert!(d.is_empty());
    assert_eq!(d.len(), 0);
}

#[test]
fn deque_push_back_from_freshly_constructed() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(1);
    assert_eq!(d.get(0), Some(&1));
}

#[test]
fn deque_push_front_from_freshly_constructed() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_front(1);
    assert_eq!(d.get(0), Some(&1));
}

#[test]
fn deque_front_back_empty() {
    let d: Deque<i32, _> = Deque::new_in(alloc());
    assert_eq!(d.front(), None);
    assert_eq!(d.back(), None);
}

#[test]
fn deque_push_back_and_get() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(10);
    d.push_back(20);
    d.push_back(30);
    assert_eq!(d.len(), 3);
    assert_eq!(d.get(0), Some(&10));
    assert_eq!(d.get(1), Some(&20));
    assert_eq!(d.get(2), Some(&30));
    assert_eq!(d.get(3), None);
}

#[test]
fn deque_push_front_and_get() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_front(30);
    d.push_front(20);
    d.push_front(10);
    assert_eq!(d.len(), 3);
    assert_eq!(d.get(0), Some(&10));
    assert_eq!(d.get(1), Some(&20));
    assert_eq!(d.get(2), Some(&30));
}

#[test]
fn deque_push_front_and_back_mixed() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(2);
    d.push_front(1);
    d.push_back(3);
    d.push_front(0);
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [0, 1, 2, 3]);
}

#[test]
fn deque_front_back_refs() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(1);
    d.push_back(2);
    d.push_back(3);
    assert_eq!(d.front(), Some(&1));
    assert_eq!(d.back(), Some(&3));
}

#[test]
fn deque_front_back_mut() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(1);
    d.push_back(2);
    *d.front_mut().unwrap() = 99;
    *d.back_mut().unwrap() = 100;
    assert_eq!(d.front(), Some(&99));
    assert_eq!(d.back(), Some(&100));
}

#[test]
fn deque_pop_back() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(1);
    d.push_back(2);
    d.push_back(3);
    assert_eq!(d.pop_back(), Some(3));
    assert_eq!(d.pop_back(), Some(2));
    assert_eq!(d.pop_back(), Some(1));
    assert_eq!(d.pop_back(), None);
    assert!(d.is_empty());
}

#[test]
fn deque_pop_front() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    d.push_back(1);
    d.push_back(2);
    d.push_back(3);
    assert_eq!(d.pop_front(), Some(1));
    assert_eq!(d.pop_front(), Some(2));
    assert_eq!(d.pop_front(), Some(3));
    assert_eq!(d.pop_front(), None);
    assert!(d.is_empty());
}

#[test]
fn deque_used_as_queue() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..8 {
        d.push_back(i);
    }
    let mut out = Vec::new();
    while let Some(v) = d.pop_front() {
        out.push(v);
    }
    assert_eq!(out, (0..8).collect::<Vec<_>>());
}

#[test]
fn deque_used_as_stack() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..8 {
        d.push_back(i);
    }
    let mut out = Vec::new();
    while let Some(v) = d.pop_back() {
        out.push(v);
    }
    assert_eq!(out, (0..8).rev().collect::<Vec<_>>());
}

#[test]
fn deque_pop_interleaved_with_push() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..4 {
        d.push_back(i);
    }
    assert_eq!(d.pop_front(), Some(0));
    d.push_back(4);
    assert_eq!(d.pop_front(), Some(1));
    d.push_back(5);
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [2, 3, 4, 5]);
}

#[test]
fn deque_get_mut() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..5 {
        d.push_back(i);
    }
    *d.get_mut(2).unwrap() = 99;
    assert_eq!(d.get(2), Some(&99));
    assert_eq!(d.get_mut(5), None);
}

#[test]
fn deque_iter_forward() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..5 {
        d.push_back(i * 10);
    }
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [0, 10, 20, 30, 40]);
}

#[test]
fn deque_iter_rev() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..5 {
        d.push_back(i);
    }
    let collected: Vec<_> = d.iter().copied().rev().collect();
    assert_eq!(collected, [4, 3, 2, 1, 0]);
}

#[test]
fn deque_iter_double_ended() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 1..=6 {
        d.push_back(i);
    }
    let mut it = d.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next_back(), Some(&6));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next_back(), Some(&5));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn deque_iter_exact_size() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..7 {
        d.push_back(i);
    }
    let mut it = d.iter();
    assert_eq!(it.len(), 7);
    it.next();
    assert_eq!(it.len(), 6);
}

#[test]
fn deque_iter_mut() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..5 {
        d.push_back(i);
    }
    for v in d.iter_mut() {
        *v *= 2;
    }
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [0, 2, 4, 6, 8]);
}

#[test]
fn deque_iter_mut_double_ended() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 1..=4 {
        d.push_back(i);
    }
    let mut it = d.iter_mut();
    *it.next().unwrap() = 10;
    *it.next_back().unwrap() = 40;
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [10, 2, 3, 40]);
}

#[test]
fn deque_into_iter_ref() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..4 {
        d.push_back(i);
    }
    let collected: Vec<_> = (&d).into_iter().copied().collect();
    assert_eq!(collected, [0, 1, 2, 3]);
}

#[test]
fn deque_into_iter_mut_ref() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..4 {
        d.push_back(i);
    }
    for v in &mut d {
        *v += 100;
    }
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [100, 101, 102, 103]);
}

#[test]
fn deque_large_push_back_in_order() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..256 {
        d.push_back(i);
    }
    assert_eq!(d.len(), 256);
    let collected: Vec<_> = d.iter().copied().collect();
    let expected: Vec<_> = (0..256).collect();
    assert_eq!(collected, expected);
}

#[test]
fn deque_large_push_front_in_order() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in (0..256).rev() {
        d.push_front(i);
    }
    assert_eq!(d.len(), 256);
    let collected: Vec<_> = d.iter().copied().collect();
    let expected: Vec<_> = (0..256).collect();
    assert_eq!(collected, expected);
}

#[test]
fn deque_large_alternating_push() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    // Interleave pushes at both ends to stress block/map growth
    for i in 0..128 {
        d.push_back(128 + i);
        d.push_front(127 - i);
    }
    assert_eq!(d.len(), 256);
    let collected: Vec<_> = d.iter().copied().collect();
    let expected: Vec<_> = (0..256).collect();
    assert_eq!(collected, expected);
}

#[test]
fn deque_large_random_access() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..256 {
        d.push_back(i * 3);
    }
    for i in 0..256 {
        assert_eq!(d.get(i), Some(&(i as i32 * 3)));
    }
}

#[test]
fn deque_no_leak_empty() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let _d: Deque<i32, _> = Deque::new_in(a.clone());
    }
    assert_eq!(a.live_count(), 0, "empty deque leaked");
}

#[test]
fn deque_no_leak_after_push_back() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut d: Deque<i32, _> = Deque::new_in(a.clone());
        for i in 0..128 {
            d.push_back(i);
        }
    }
    assert_eq!(a.live_count(), 0, "deque leaked after push_back");
}

#[test]
fn deque_no_leak_after_push_front() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut d: Deque<i32, _> = Deque::new_in(a.clone());
        for i in 0..128 {
            d.push_front(i);
        }
    }
    assert_eq!(a.live_count(), 0, "deque leaked after push_front");
}

#[test]
fn deque_no_leak_after_pop_all() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut d: Deque<i32, _> = Deque::new_in(a.clone());
        for i in 0..64 {
            d.push_back(i);
        }
        while d.pop_front().is_some() {}
    }
    assert_eq!(a.live_count(), 0, "deque leaked after popping all elements");
}

#[test]
fn deque_drop_calls_element_destructors() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut d: Deque<DropCount, _> = Deque::new_in(a.clone());
        for i in 0..32 {
            d.push_back(DropCount::new(&drop_count, i));
        }
        assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        32,
        "not all elements dropped"
    );
    assert_eq!(a.live_count(), 0);
}

#[test]
fn deque_pop_front_calls_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut d: Deque<DropCount, _> = Deque::new_in(a.clone());
    d.push_back(DropCount::new(&drop_count, 1));
    d.push_back(DropCount::new(&drop_count, 2));
    drop(d.pop_front());
    assert_eq!(drop_count.load(Ordering::Relaxed), 1);
    drop(d);
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn deque_pop_back_calls_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut d: Deque<DropCount, _> = Deque::new_in(a.clone());
    d.push_back(DropCount::new(&drop_count, 1));
    d.push_back(DropCount::new(&drop_count, 2));
    drop(d.pop_back());
    assert_eq!(drop_count.load(Ordering::Relaxed), 1);
    drop(d);
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn deque_drop_calls_destructors_across_blocks() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    const N: usize = 256;
    {
        let mut d: Deque<DropCount, _> = Deque::new_in(a.clone());
        for i in 0..N {
            d.push_back(DropCount::new(&drop_count, i as i32));
        }
        assert_eq!(d.len(), N);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        N,
        "not all elements dropped across blocks"
    );
    assert_eq!(a.live_count(), 0);
}

#[test]
fn deque_clear_functional() {
    let mut d: Deque<i32, _> = Deque::new_in(alloc());
    for i in 0..50 {
        d.push_back(i);
    }
    assert_eq!(d.len(), 50);

    d.clear();
    assert_eq!(d.len(), 0);
    assert!(d.is_empty());
    assert_eq!(d.front(), None);
    assert_eq!(d.back(), None);

    // Verify the deque can be successfully reused after being cleared
    for i in 0..5 {
        d.push_back(i * 10);
    }
    assert_eq!(d.len(), 5);
    let collected: Vec<_> = d.iter().copied().collect();
    assert_eq!(collected, [0, 10, 20, 30, 40]);
}

#[test]
fn deque_no_leak_after_clear() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut d: Deque<i32, _> = Deque::new_in(a.clone());
        for i in 0..128 {
            d.push_back(i);
        }
        d.clear();
    }
    assert_eq!(a.live_count(), 0, "deque leaked after clear");
}

#[test]
fn deque_zst() {
    let mut d: Deque<(), _> = Deque::new_in(alloc());
    for _ in 0..16 {
        d.push_back(());
    }
    assert_eq!(d.len(), 16);
    assert_eq!(d.iter().count(), 16);
    for _ in 0..16 {
        assert_eq!(d.pop_front(), Some(()));
    }
    assert!(d.is_empty());
}
