mod common;

use fromsoftware_shared_stl::{Map, MultiMap, MultiSet, Pair, Set};

use std::{
    ops::Bound,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::common::{DropCount, StdAlloc};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn std_alloc() -> StdAlloc {
    StdAlloc::new(&COUNTER)
}

#[test]
fn set_insert_and_find() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    assert!(set.insert(5));
    assert!(set.insert(3));
    assert!(set.insert(7));

    assert_eq!(set.find(&3), Some(&3));
    assert_eq!(set.find(&5), Some(&5));
    assert_eq!(set.find(&7), Some(&7));
    assert_eq!(set.find(&99), None);
}

#[test]
fn set_insert_duplicate_returns_false() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    assert!(set.insert(42));
    assert!(!set.insert(42));
    assert_eq!(set.len(), 1);
}

#[test]
fn set_contains() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    set.insert(10);
    assert!(set.contains(&10));
    assert!(!set.contains(&11));
}

#[test]
fn set_remove() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    set.insert(1);
    set.insert(2);
    set.insert(3);
    assert_eq!(set.remove(&2), Some(2));
    assert_eq!(set.remove(&2), None);
    assert_eq!(set.len(), 2);
}

#[test]
fn set_iter_in_order() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    for v in [5, 1, 4, 2, 3] {
        set.insert(v);
    }
    let collected: Vec<_> = set.iter().copied().collect();
    assert_eq!(collected, [1, 2, 3, 4, 5]);
}

#[test]
fn set_pop_min_max() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    for v in [3, 1, 2] {
        set.insert(v);
    }
    assert_eq!(set.pop_min(), Some(1));
    assert_eq!(set.pop_max(), Some(3));
    assert_eq!(set.len(), 1);
}

#[test]
fn set_lower_upper_bound() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    for v in [1, 3, 5, 7, 9] {
        set.insert(v);
    }

    assert_eq!(set.lower_bound(Bound::Included(&4)), Some(&5));
    assert_eq!(set.lower_bound(Bound::Excluded(&3)), Some(&5));
    assert_eq!(set.upper_bound(Bound::Included(&6)), Some(&5));
    assert_eq!(set.upper_bound(Bound::Excluded(&5)), Some(&3));
    assert_eq!(set.lower_bound(Bound::Unbounded), Some(&1));
    assert_eq!(set.upper_bound(Bound::Unbounded), Some(&9));
}

#[test]
fn set_get_or_insert() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    assert_eq!(set.get_or_insert(10), &10);
    assert_eq!(set.get_or_insert(10), &10);
    assert_eq!(set.len(), 1);
}

#[test]
fn set_no_leak() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut set: Set<i32, _> = Set::new_in(a.clone());
        for v in 0..100 {
            set.insert(v);
        }
    }
    assert_eq!(a.live_count(), 0, "leaked allocations");
}

#[test]
fn map_insert_and_find() {
    let mut map: Map<&str, i32, _> = Map::new_in(std_alloc());
    assert_eq!(map.insert("a", 1), None);
    assert_eq!(map.insert("b", 2), None);
    assert_eq!(map.find(&"a"), Some(&1));
    assert_eq!(map.find(&"b"), Some(&2));
    assert_eq!(map.find(&"c"), None);
}

#[test]
fn map_insert_replaces_existing() {
    let mut map: Map<i32, i32, _> = Map::new_in(std_alloc());
    assert_eq!(map.insert(1, 10), None);
    assert_eq!(map.insert(1, 20), Some(10));
    assert_eq!(map.find(&1), Some(&20));
    assert_eq!(map.len(), 1);
}

#[test]
fn map_try_insert() {
    let mut map: Map<i32, i32, _> = Map::new_in(std_alloc());
    assert!(map.try_insert(1, 10).is_ok());
    assert!(map.try_insert(1, 20).is_err());
    assert_eq!(map.find(&1), Some(&10));
}

#[test]
fn map_find_mut() {
    let mut map: Map<i32, i32, _> = Map::new_in(std_alloc());
    map.insert(1, 10);
    *map.find_mut(&1).unwrap() = 99;
    assert_eq!(map.find(&1), Some(&99));
}

#[test]
fn map_remove() {
    let mut map: Map<i32, i32, _> = Map::new_in(std_alloc());
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(
        map.remove(&1),
        Some(Pair {
            first: 1,
            second: 10
        })
    );
    assert_eq!(map.remove(&1), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn map_iter_in_order() {
    let mut map: Map<i32, i32, _> = Map::new_in(std_alloc());
    for (k, v) in [(3, 30), (1, 10), (2, 20)] {
        map.insert(k, v);
    }
    let keys: Vec<_> = map.iter().map(|p| p.first).collect();
    assert_eq!(keys, [1, 2, 3]);
}

#[test]
fn map_no_leak() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut map: Map<i32, String, _> = Map::new_in(a.clone());
        for i in 0..50 {
            map.insert(i, format!("val{i}"));
        }
    }
    assert_eq!(a.live_count(), 0, "leaked allocations");
}

#[test]
fn multiset_allows_duplicates() {
    let mut ms: MultiSet<i32, _> = MultiSet::new_in(std_alloc());
    ms.insert(3);
    ms.insert(3);
    ms.insert(3);
    assert_eq!(ms.len(), 3);
}

#[test]
fn multiset_find_returns_all() {
    let mut ms: MultiSet<i32, _> = MultiSet::new_in(std_alloc());
    for v in [1, 2, 2, 2, 3] {
        ms.insert(v);
    }
    let found: Vec<_> = ms.find(&2).copied().collect();
    assert_eq!(found, [2, 2, 2]);
}

#[test]
fn multiset_remove_all() {
    let mut ms: MultiSet<i32, _> = MultiSet::new_in(std_alloc());
    for v in [1, 2, 2, 2, 3] {
        ms.insert(v);
    }
    assert_eq!(ms.remove_all(&2), 3);
    assert_eq!(ms.len(), 2);
    assert_eq!(ms.find(&2).count(), 0);
}

#[test]
fn multiset_iter_in_order() {
    let mut ms: MultiSet<i32, _> = MultiSet::new_in(std_alloc());
    for v in [3, 1, 2, 1, 3] {
        ms.insert(v);
    }
    let collected: Vec<_> = ms.iter().copied().collect();
    assert_eq!(collected, [1, 1, 2, 3, 3]);
}

#[test]
fn multimap_allows_duplicate_keys() {
    let mut mm: MultiMap<i32, &str, _> = MultiMap::new_in(std_alloc());
    mm.insert(1, "a");
    mm.insert(1, "b");
    mm.insert(1, "c");
    assert_eq!(mm.len(), 3);
}

#[test]
fn multimap_find_returns_all_values() {
    let mut mm: MultiMap<i32, i32, _> = MultiMap::new_in(std_alloc());
    for (k, v) in [(1, 10), (2, 20), (1, 11), (1, 12)] {
        mm.insert(k, v);
    }
    let vals: Vec<_> = mm.find(&1).copied().collect();
    assert_eq!(vals, [10, 11, 12]);
}

#[test]
fn multimap_remove_all() {
    let mut mm: MultiMap<i32, i32, _> = MultiMap::new_in(std_alloc());
    for v in [10, 11, 12] {
        mm.insert(1, v);
    }
    mm.insert(2, 20);
    assert_eq!(mm.remove_all(&1), 3);
    assert_eq!(mm.len(), 1);
}

#[test]
fn set_large_insertion_sorted() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    // Insert in reverse order to test rebalancing
    for v in (0..256).rev() {
        set.insert(v);
    }
    assert_eq!(set.len(), 256);
    let collected: Vec<_> = set.iter().copied().collect();
    let expected: Vec<_> = (0..256).collect();
    assert_eq!(collected, expected);
}

#[test]
fn set_insert_remove_all() {
    let mut set: Set<i32, _> = Set::new_in(std_alloc());
    for v in 0..64 {
        set.insert(v);
    }
    for v in 0..64 {
        assert_eq!(set.remove(&v), Some(v));
    }
    assert!(set.is_empty());
}

#[test]
fn map_large_no_leak() {
    static C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&C);
    {
        let mut map: Map<i32, i32, _> = Map::new_in(a.clone());
        for i in 0..256 {
            map.insert(i, i * 2);
        }

        for i in (0..256).step_by(2) {
            map.remove(&i);
        }

        for i in (1..256).step_by(2) {
            assert_eq!(map.find(&i), Some(&(i * 2)));
        }
    }
    assert_eq!(a.live_count(), 0);
}

#[test]
fn set_drop_calls_value_destructors() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut set: Set<DropCount, _> = Set::new_in(a.clone());
        for i in 0..16 {
            set.insert(DropCount::new(&drop_count, i));
        }
        assert_eq!(
            drop_count.load(Ordering::Relaxed),
            0,
            "no drops before tree drop"
        );
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        16,
        "not all values were dropped"
    );
    assert_eq!(a.live_count(), 0, "allocations leaked");
}

#[test]
fn set_remove_calls_value_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut set: Set<DropCount, _> = Set::new_in(a.clone());
    for i in 0..8 {
        set.insert(DropCount::new(&drop_count, i));
    }
    let key = DropCount::new(&drop_count, 0);
    set.remove(&key);
    // remove returns the value which is then dropped at end of statement
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        1,
        "removed value not dropped"
    );
    drop(key);
    drop(set);
    // prev 1 + remaining 7 + the key passed to remove
    assert_eq!(drop_count.load(Ordering::Relaxed), 1 + 7 + 1);
}

#[test]
fn set_pop_min_max_calls_destructor() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut set: Set<DropCount, _> = Set::new_in(a.clone());
    for i in 0..4 {
        set.insert(DropCount::new(&drop_count, i));
    }
    drop(set.pop_min()); // value dropped at end of statement
    assert_eq!(drop_count.load(Ordering::Relaxed), 1);
    drop(set.pop_max());
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    drop(set); // remaining 2
    assert_eq!(drop_count.load(Ordering::Relaxed), 4);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn set_large_drop_all_destructors_called() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    const N: i32 = 256;
    {
        let mut set: Set<DropCount, _> = Set::new_in(a.clone());
        // Insert in reverse to test rebalancing
        for i in (0..N).rev() {
            set.insert(DropCount::new(&drop_count, i));
        }
        assert_eq!(set.len(), N as usize);
        assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        N as usize,
        "not all values dropped"
    );
    assert_eq!(a.live_count(), 0, "allocations leaked");
}

#[test]
fn map_insert_replace_drops_old_value() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut map: Map<i32, DropCount, _> = Map::new_in(a.clone());
    map.insert(1, DropCount::new(&drop_count, 10));
    assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    // Replace old value returned and immediately drop
    drop(map.insert(1, DropCount::new(&drop_count, 20)));
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        1,
        "replaced value not dropped"
    );
    drop(map); // drops the current value (20)
    assert_eq!(drop_count.load(Ordering::Relaxed), 2);
    assert_eq!(a.live_count(), 0);
}

#[test]
fn map_drop_calls_all_value_destructors() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    {
        let mut map: Map<i32, DropCount, _> = Map::new_in(a.clone());
        for i in 0..32 {
            map.insert(i, DropCount::new(&drop_count, i));
        }
        assert_eq!(drop_count.load(Ordering::Relaxed), 0);
    }
    assert_eq!(
        drop_count.load(Ordering::Relaxed),
        32,
        "not all map values dropped"
    );
    assert_eq!(a.live_count(), 0);
}

#[test]
fn multiset_remove_all_drops_all_matching() {
    let drop_count = AtomicUsize::new(0);
    static ALLOC_C: AtomicUsize = AtomicUsize::new(0);
    let a = StdAlloc::new(&ALLOC_C);
    let mut ms: MultiSet<DropCount, _> = MultiSet::new_in(a.clone());
    for _ in 0..4 {
        ms.insert(DropCount::new(&drop_count, 42));
    }
    ms.insert(DropCount::new(&drop_count, 1));
    ms.insert(DropCount::new(&drop_count, 2));

    let removed = ms.remove_all(&DropCount::new(&drop_count, 42));
    assert_eq!(removed, 4);
    // The 4 removed values + the temporary key passed to remove_all
    assert_eq!(drop_count.load(Ordering::Relaxed), 4 + 1);
    drop(ms); // remaining 2
    assert_eq!(drop_count.load(Ordering::Relaxed), 4 + 1 + 2);
    assert_eq!(a.live_count(), 0);
}
