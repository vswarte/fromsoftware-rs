use crate::{Pair, allocator::*};
use std::{cmp::Ordering, iter::FusedIterator, mem::MaybeUninit, ptr::NonNull};

/// Comparator trait for use in MSVC `std::tree` [`RbTree`]
pub trait TreeComparator<V> {
    fn lt(&self, a: &V, b: &V) -> bool;

    #[inline]
    fn eq(&self, a: &V, b: &V) -> bool {
        !self.lt(a, b) && !self.lt(b, a)
    }
}

/// Default [`RbTree`] comparator, equivalent to MSVC [`std::less<V>`]
///
/// [`std::less<V>`]: https://en.cppreference.com/w/cpp/utility/functional/less.html
pub struct Less;

impl<V: Ord> TreeComparator<V> for Less {
    #[inline]
    fn lt(&self, a: &V, b: &V) -> bool {
        a < b
    }
}

/// Comparator for `Map<K, V>` that orders by key only,
/// equivalent to MSVC [`std::less<K>`] applied to [`std::pair::first`]
///
/// [`std::less<K>`]: https://en.cppreference.com/w/cpp/utility/functional/less.html
/// [`std::pair::first`]: https://en.cppreference.com/w/cpp/utility/pair.html
pub struct KeyLess;

impl<K: Ord, V> TreeComparator<Pair<K, V>> for KeyLess {
    #[inline]
    fn lt(&self, a: &Pair<K, V>, b: &Pair<K, V>) -> bool {
        a.first < b.first
    }
}

pub trait TreeComparatorKey<K, V>: TreeComparator<V> {
    fn lt_key_val(&self, key: &K, val: &V) -> bool;
    fn lt_val_key(&self, val: &V, key: &K) -> bool;

    #[inline]
    fn eq_key(&self, key: &K, val: &V) -> bool {
        !self.lt_key_val(key, val) && !self.lt_val_key(val, key)
    }
}

impl<K: Ord, V> TreeComparatorKey<K, Pair<K, V>> for KeyLess {
    #[inline]
    fn lt_key_val(&self, key: &K, val: &Pair<K, V>) -> bool {
        key < &val.first
    }
    #[inline]
    fn lt_val_key(&self, val: &Pair<K, V>, key: &K) -> bool {
        val.first < *key
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RbColor {
    Red = 0,
    Black = 1,
}

#[repr(C)]
pub struct RbNode<V> {
    left: NonNull<RbNode<V>>,
    parent: NonNull<RbNode<V>>,
    right: NonNull<RbNode<V>>,
    color: RbColor,
    is_nil: bool,
    value: MaybeUninit<V>,
}

/// A guaranteed valid, non-null pointer to an [`RbNode`]
#[repr(transparent)]
struct NodePtr<V>(NonNull<RbNode<V>>);

impl<V> From<NonNull<RbNode<V>>> for NodePtr<V> {
    fn from(p: NonNull<RbNode<V>>) -> Self {
        NodePtr(p)
    }
}

impl<V> PartialEq for NodePtr<V> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}
impl<V> Eq for NodePtr<V> {}

impl<V> Copy for NodePtr<V> {}
impl<V> Clone for NodePtr<V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<V> NodePtr<V> {
    #[inline]
    fn get(&self) -> &RbNode<V> {
        unsafe { self.0.as_ref() }
    }
    #[inline]
    /// # Safety
    ///
    /// No other reference to the node this pointer points to may exist
    /// at the same time. Since `NodePtr` is `Copy`, the borrow checker
    /// cannot enforce this, the caller must guarantee it
    unsafe fn get_mut(&mut self) -> &mut RbNode<V> {
        unsafe { self.0.as_mut() }
    }
    #[inline]
    fn as_ptr(&self) -> *mut RbNode<V> {
        self.0.as_ptr()
    }
    #[inline]
    fn left(&self) -> NodePtr<V> {
        NodePtr(self.get().left)
    }
    #[inline]
    fn right(&self) -> NodePtr<V> {
        NodePtr(self.get().right)
    }
    #[inline]
    fn parent(&self) -> NodePtr<V> {
        NodePtr(self.get().parent)
    }
    #[inline]
    fn is_nil(&self) -> bool {
        self.get().is_nil
    }
    #[inline]
    fn is_red(&self) -> bool {
        self.get().color == RbColor::Red
    }
    #[inline]
    fn set_red(&mut self) {
        unsafe { self.get_mut().color = RbColor::Red };
    }
    #[inline]
    fn set_black(&mut self) {
        unsafe { self.get_mut().color = RbColor::Black };
    }
    #[inline]
    fn set_left(&mut self, n: NodePtr<V>) {
        unsafe { self.get_mut().left = n.0 };
    }
    #[inline]
    fn set_right(&mut self, n: NodePtr<V>) {
        unsafe { self.get_mut().right = n.0 };
    }
    #[inline]
    fn set_parent(&mut self, n: NodePtr<V>) {
        unsafe { self.get_mut().parent = n.0 };
    }
    #[inline]
    fn is_left_child(&self) -> bool {
        *self == self.parent().left()
    }
    #[inline]
    fn is_right_child(&self) -> bool {
        *self == self.parent().right()
    }

    /// Splices `replacement` into this node's position in the tree.
    ///
    /// Sets `replacement`'s parent pointer to `self`'s parent, then updates
    /// the parent's child pointer (or the root pointer via `head`) to point at
    /// `replacement` instead of `self`. Does not update `self`'s own pointers,
    /// so `self` should be considered detached after this call.
    ///
    /// # Safety
    ///
    /// `self` must be a live non-sentinel node belonging to the tree rooted at `head`.
    /// `replacement` must be a valid node (may be the sentinel).
    unsafe fn replace_in_parent(&mut self, mut replacement: NodePtr<V>, head: NodePtr<V>) {
        replacement.set_parent(self.parent());
        unsafe {
            if self.parent().as_ptr() == head.as_ptr() {
                self.parent().get_mut().parent = replacement.0;
            } else if self.is_left_child() {
                self.parent().get_mut().left = replacement.0;
            } else {
                self.parent().get_mut().right = replacement.0;
            }
        }
    }

    fn leftmost(&self) -> NodePtr<V> {
        let mut n = *self;
        while !n.left().is_nil() {
            n = n.left();
        }
        n
    }

    fn rightmost(&self) -> NodePtr<V> {
        let mut n = *self;
        while !n.right().is_nil() {
            n = n.right();
        }
        n
    }
}

#[repr(C)]
/// Implementation of a basic MSVC C++ Red-Black Tree
///
/// # References
///
/// - [Raymond Chen's breakdown of `xtree`]
///
/// [Raymond Chen's breakdown of `xtree`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
pub struct RbTree<V, A: Allocator, C: Sized, const UNIQUE: bool = true> {
    comparator: C,
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    allocator: A,
    head: NonNull<RbNode<V>>,
    size: usize,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    allocator: A,
}

/// Implementation of MSVC C++ `std::map`
///
/// # References
///
/// - [cppreference - `std::map`]
/// - [Raymond Chen's breakdown of `std::map`]
///
/// [cppreference - `std::map`]: https://en.cppreference.com/w/cpp/container/map.html
/// [Raymond Chen's breakdown of `std::map`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
pub type Map<K, V, A, C = KeyLess> = RbTree<Pair<K, V>, A, C, true>;
/// Implementation of MSVC C++ `std::set`
///
/// # References
///
/// - [cppreference - `std::set`]
/// - [Raymond Chen's breakdown of `std::set`]
///
/// [cppreference - `std::set`]: https://en.cppreference.com/w/cpp/container/set.html
/// [Raymond Chen's breakdown of `std::set`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
pub type Set<K, A, C = Less> = RbTree<K, A, C, true>;
/// Implementation of MSVC C++ `std:multimap`
///
/// # References
///
/// - [cppreference - `std::multimap`]
/// - [Raymond Chen's breakdown of `std::multimap`]
///
/// [cppreference - `std::multimap`]: https://en.cppreference.com/w/cpp/container/multimap.html
/// [Raymond Chen's breakdown of `std::multimap`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
pub type MultiMap<K, V, A, C = KeyLess> = RbTree<Pair<K, V>, A, C, false>;
/// Implementation of MSVC C++ `std::multiset`
///
/// # References
///
/// - [cppreference - `std::multiset`]
/// - [Raymond Chen's breakdown of `std::multiset`]
///
/// [cppreference - `std::multiset`]: https://en.cppreference.com/w/cpp/container/multiset.html
/// [Raymond Chen's breakdown of `std::multiset`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
pub type MultiSet<K, A, C = Less> = RbTree<K, A, C, false>;

impl<V, A: Allocator, C: TreeComparator<V>, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    pub fn contains(&self, value: &V) -> bool {
        self.find_node(value).is_some()
    }

    pub fn find(&self, value: &V) -> Option<&V> {
        self.find_node(value)
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    pub fn find_mut(&mut self, value: &V) -> Option<&mut V> {
        self.find_node(value)
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    /// Shared traversal for `lower_bound` and `upper_bound`.
    /// `go_left` determines whether to track the current node as a candidate
    /// and descend left, or descend right unconditionally
    fn bound_node_by<Q>(&self, query: &Q, go_left: impl Fn(&Q, &V) -> bool) -> Option<NodePtr<V>> {
        unsafe {
            let mut node = NodePtr(self.head.as_ref().parent);
            let mut result = None;
            loop {
                if node.is_nil() {
                    return result;
                }
                let val = (*node.as_ptr()).value.assume_init_ref();
                if go_left(query, val) {
                    result = Some(node);
                    node = node.left();
                } else {
                    node = node.right();
                }
            }
        }
    }

    /// First element `>= value`
    pub fn lower_bound(&self, value: &V) -> Option<&V> {
        // Go left when node_val >= value, so when node_val is NOT less than value
        self.bound_node_by(value, |v, node_val| !self.comparator.lt(node_val, v))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    /// First element `>= value`
    pub fn lower_bound_mut(&mut self, value: &V) -> Option<&mut V> {
        self.bound_node_by(value, |v, node_val| !self.comparator.lt(node_val, v))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    /// First element `> value`
    pub fn upper_bound(&self, value: &V) -> Option<&V> {
        // Go left only when node_val > value, so when value is strictly less
        self.bound_node_by(value, |v, node_val| self.comparator.lt(v, node_val))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    /// First element `> value`
    pub fn upper_bound_mut(&mut self, value: &V) -> Option<&mut V> {
        self.bound_node_by(value, |v, node_val| self.comparator.lt(v, node_val))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    /// Insert a value, returning a reference to it
    ///
    /// Duplicate values are ignored for [`UNIQUE: true`]
    pub fn insert(&mut self, value: V) -> &V {
        let mut head = NodePtr(self.head);
        let mut parent = head;
        let mut node = head.parent();
        let mut insert_left = true;

        while !node.is_nil() {
            parent = node;
            let node_val = unsafe { (*node.as_ptr()).value.assume_init_ref() };

            if self.comparator.lt(&value, node_val) {
                insert_left = true;
                node = node.left();
            } else if !UNIQUE || self.comparator.lt(node_val, &value) {
                // false: always go right
                // UNIQUE: true: only go right if node_val < value
                insert_left = false;
                node = node.right();
            } else {
                // Return existing if UNIQUE: true and equal
                return unsafe { (*node.as_ptr()).value.assume_init_ref() };
            }
        }

        let new_node = NodePtr(unsafe { self.allocator.allocate::<RbNode<V>>().cast() });
        unsafe {
            std::ptr::write(
                new_node.as_ptr(),
                RbNode {
                    left: head.0,
                    parent: parent.0,
                    right: head.0,
                    color: RbColor::Red,
                    is_nil: false,
                    value: MaybeUninit::new(value),
                },
            );
        }

        if parent == head {
            unsafe {
                head.get_mut().parent = new_node.0;
                head.get_mut().left = new_node.0;
                head.get_mut().right = new_node.0;
            }
        } else if insert_left {
            parent.set_left(new_node);
            if parent == head.left() {
                unsafe { head.get_mut().left = new_node.0 };
            }
        } else {
            parent.set_right(new_node);
            if parent == head.right() {
                unsafe { head.get_mut().right = new_node.0 };
            }
        }

        self.size = self.size.checked_add(1).expect("tree size overflow");
        rb_insert_fixup(new_node, head);
        // Safety: only the root node should have an uninitialized value
        unsafe { (*new_node.as_ptr()).value.assume_init_ref() }
    }

    pub fn remove(&mut self, value: &V) -> Option<V> {
        let node = self.find_node(value)?;
        Some(unsafe { self.extract_node(node) })
    }

    pub fn pop_min(&mut self) -> Option<V> {
        (self.size > 0).then(|| unsafe { self.extract_node(NodePtr(self.head).left()) })
    }

    pub fn pop_max(&mut self) -> Option<V> {
        (self.size > 0).then(|| unsafe { self.extract_node(NodePtr(self.head).right()) })
    }

    fn find_node(&self, value: &V) -> Option<NodePtr<V>> {
        self.bound_node_by(value, |v, nv| !self.comparator.lt(nv, v))
            .filter(|n| {
                self.comparator
                    .eq(value, unsafe { (*n.as_ptr()).value.assume_init_ref() })
            })
    }

    /// Port of MSVC `_Tree_val::_Extract` + `_Tree_node::_Freenode`
    ///
    /// # Safety
    /// `erased` must be a live non-sentinel node belonging to this tree
    unsafe fn extract_node(&mut self, mut erased: NodePtr<V>) -> V {
        let mut head = NodePtr(self.head);
        let successor = rb_successor(erased, head);

        let (fixnode, fixparent) = unsafe {
            if erased.left().is_nil() || erased.right().is_nil() {
                let mut fix = if erased.left().is_nil() {
                    erased.right()
                } else {
                    erased.left()
                };
                erased.replace_in_parent(fix, head);
                if !fix.is_nil() {
                    fix.set_parent(erased.parent());
                }
                (fix, erased.parent())
            } else {
                let mut succ = successor;
                let mut fix = succ.right();

                let fixparent = if succ == erased.right() {
                    succ
                } else {
                    let mut sp = succ.parent();
                    if !fix.is_nil() {
                        fix.set_parent(sp);
                    }
                    sp.get_mut().left = fix.0;
                    succ.set_right(erased.right());
                    erased.right().set_parent(succ);
                    sp
                };

                succ.set_left(erased.left());
                erased.left().set_parent(succ);
                erased.replace_in_parent(succ, head);
                succ.set_parent(erased.parent());

                let (ec, sc) = (erased.get().color, succ.get().color);

                succ.get_mut().color = ec;
                erased.get_mut().color = sc;

                (fix, fixparent)
            }
        };

        let set = |ptr: &mut NonNull<RbNode<V>>, n: NodePtr<V>| *ptr = n.0;
        if head.parent() == erased {
            unsafe { set(&mut head.get_mut().parent, fixnode) };
        }
        if head.left() == erased {
            let m = if fixnode.is_nil() {
                fixparent
            } else {
                fixnode.leftmost()
            };
            unsafe { set(&mut head.get_mut().left, m) };
        }
        if head.right() == erased {
            let m = if fixnode.is_nil() {
                fixparent
            } else {
                fixnode.rightmost()
            };
            unsafe { set(&mut head.get_mut().right, m) };
        }

        if erased.get().color == RbColor::Black {
            rb_erase_fixup(fixnode, fixparent, head);
        }

        self.size -= 1;
        let value = unsafe { (*erased.as_ptr()).value.assume_init_read() };
        unsafe { self.allocator.deallocate_raw(erased.as_ptr() as _) };
        value
    }
}

impl<V, A: Allocator, C: Sized, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    /// Inorder iterator, yields values in ascending comparator order
    pub fn iter(&self) -> RbTreeIter<'_, V> {
        let head = NodePtr(self.head);
        RbTreeIter {
            head,
            current: head.left(),
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> RbTreeIterMut<'_, V> {
        let head = NodePtr(self.head);
        RbTreeIterMut {
            head,
            current: head.left(),
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
    }

    /// O(log n) seach by filter function
    pub fn find_by(&self, f: impl FnMut(&V) -> Ordering) -> Option<&V> {
        rb_find_by(NodePtr(self.head), f).map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    /// O(log n) search by ordering, returns mutable reference
    pub fn find_by_mut(&mut self, f: impl FnMut(&V) -> Ordering) -> Option<&mut V> {
        rb_find_by(NodePtr(self.head), f).map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<K, V, A: Allocator, C: TreeComparatorKey<K, Pair<K, V>>, const UNIQUE: bool>
    RbTree<Pair<K, V>, A, C, UNIQUE>
{
    fn find_node_key(&self, key: &K) -> Option<NodePtr<Pair<K, V>>> {
        self.bound_node_by(key, |k, val| !self.comparator.lt_val_key(val, k))
            .filter(|n| {
                self.comparator
                    .eq_key(key, unsafe { (*n.as_ptr()).value.assume_init_ref() })
            })
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.find_node_key(key).is_some()
    }

    pub fn find_key(&self, key: &K) -> Option<&V> {
        self.find_node_key(key)
            .map(|n| unsafe { &(*n.as_ptr()).value.assume_init_ref().second })
    }

    pub fn find_key_mut(&mut self, key: &K) -> Option<&mut V> {
        self.find_node_key(key)
            .map(|n| unsafe { &mut (*n.as_ptr()).value.assume_init_mut().second })
    }

    pub fn lower_bound_key(&self, key: &K) -> Option<&Pair<K, V>> {
        self.bound_node_by(key, |k, val| !self.comparator.lt_val_key(val, k))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    pub fn lower_bound_key_mut(&mut self, key: &K) -> Option<&mut Pair<K, V>> {
        self.bound_node_by(key, |k, val| !self.comparator.lt_val_key(val, k))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    pub fn upper_bound_key(&self, key: &K) -> Option<&Pair<K, V>> {
        self.bound_node_by(key, |k, val| self.comparator.lt_key_val(k, val))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    pub fn upper_bound_key_mut(&mut self, key: &K) -> Option<&mut Pair<K, V>> {
        self.bound_node_by(key, |k, val| self.comparator.lt_key_val(k, val))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }

    pub fn remove_key(&mut self, key: &K) -> Option<Pair<K, V>> {
        let node = self.find_node_key(key)?;
        Some(unsafe { self.extract_node(node) })
    }
}

impl<K, V, A: Allocator, C: Sized, const UNIQUE: bool> RbTree<Pair<K, V>, A, C, UNIQUE> {
    pub fn find_key_by(&self, mut f: impl FnMut(&K) -> Ordering) -> Option<&Pair<K, V>> {
        rb_find_by(NodePtr(self.head), |v| f(&v.first))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_ref() })
    }

    pub fn find_key_by_mut(
        &mut self,
        mut f: impl FnMut(&K) -> Ordering,
    ) -> Option<&mut Pair<K, V>> {
        rb_find_by(NodePtr(self.head), |v| f(&v.first))
            .map(|n| unsafe { (*n.as_ptr()).value.assume_init_mut() })
    }
}

impl<K, V, A: Allocator, C: Sized> RbTree<Pair<K, V>, A, C, false> {
    pub fn find_multi_key_by(
        &self,
        mut f: impl FnMut(&K) -> Ordering,
    ) -> impl Iterator<Item = &Pair<K, V>> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, |v| f(&v.first));
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| f(&v.first) == Ordering::Equal)
    }

    pub fn find_multi_key_by_mut(
        &mut self,
        mut f: impl FnMut(&K) -> Ordering,
    ) -> impl Iterator<Item = &mut Pair<K, V>> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, |v| f(&v.first));
        RbTreeIterMut {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| f(&v.first) == Ordering::Equal)
    }
}

impl<V, A: Allocator, C: TreeComparator<V>> RbTree<V, A, C, false> {
    /// Removes all elements equivalent to `value`, returns the count removed
    pub fn remove_all(&mut self, value: &V) -> usize {
        std::iter::from_fn(|| self.remove(value)).count()
    }

    /// Iterator over all elements equivalent to `value`
    pub fn find_multi(&self, value: &V) -> impl Iterator<Item = &V> {
        let head = NodePtr(self.head);
        let start = self
            .bound_node_by(value, |v, nv| !self.comparator.lt(nv, v))
            .unwrap_or(head);
        let end = self.upper_bound(value);
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| end.is_none_or(|e| !std::ptr::eq(*v, e)))
    }

    pub fn find_multi_by(&self, mut f: impl FnMut(&V) -> Ordering) -> impl Iterator<Item = &V> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, &mut f);
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| f(v) == Ordering::Equal)
    }

    pub fn find_multi_by_mut(
        &mut self,
        mut f: impl FnMut(&V) -> Ordering,
    ) -> impl Iterator<Item = &mut V> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, &mut f);
        RbTreeIterMut {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| f(v) == Ordering::Equal)
    }
}

impl<K, V, A: Allocator, C: TreeComparatorKey<K, Pair<K, V>>> RbTree<Pair<K, V>, A, C, false> {
    /// Removes all elements with `K` equals `key`, returns the count removed
    pub fn remove_all_key(&mut self, key: &K) -> usize {
        std::iter::from_fn(|| self.remove_key(key)).count()
    }

    pub fn find_multi_key(&self, key: &K) -> impl Iterator<Item = &Pair<K, V>> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, |v| {
            if self.comparator.lt_key_val(key, v) {
                Ordering::Greater
            } else if self.comparator.lt_val_key(v, key) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| {
            !self.comparator.lt_key_val(key, v) && !self.comparator.lt_val_key(v, key)
        })
    }

    pub fn find_multi_key_mut(&mut self, key: &K) -> impl Iterator<Item = &mut Pair<K, V>> {
        let head = NodePtr(self.head);
        let start = rb_find_multi_start_by(head, |v| {
            if self.comparator.lt_key_val(key, v) {
                Ordering::Greater
            } else if self.comparator.lt_val_key(v, key) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        RbTreeIterMut {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(move |v| {
            !self.comparator.lt_key_val(key, v) && !self.comparator.lt_val_key(v, key)
        })
    }
}

impl<V, A: Allocator, C: TreeComparator<V>> RbTree<V, A, C, true> {
    /// Returns the existing element if already present, otherwise inserts and
    /// returns a reference to the new value.
    pub fn get_or_insert(&mut self, value: V) -> &V {
        self.insert(value)
    }
}

pub struct RbTreeIter<'a, V> {
    head: NodePtr<V>,
    current: NodePtr<V>,
    remaining: usize,
    _marker: std::marker::PhantomData<&'a V>,
}

impl<'a, V> Iterator for RbTreeIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<&'a V> {
        if self.remaining == 0 {
            return None;
        }
        debug_assert!(!self.current.is_nil(), "iterator walked onto sentinel");

        // SAFETY: `current` is a valid non-nil node with an initialized value.
        // The lifetime 'a ties this iterator to the tree borrow,
        // so the tree cannot be mutated or dropped while
        // this iterator is alive
        let value = unsafe { (*self.current.as_ptr()).value.assume_init_ref() };
        self.remaining -= 1;
        self.current = rb_successor(self.current, self.head);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, V> ExactSizeIterator for RbTreeIter<'a, V> {}
impl<'a, V> FusedIterator for RbTreeIter<'a, V> {}

pub struct RbTreeIterMut<'a, V> {
    head: NodePtr<V>,
    current: NodePtr<V>,
    remaining: usize,
    _marker: std::marker::PhantomData<&'a mut V>,
}

impl<'a, V> Iterator for RbTreeIterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<&'a mut V> {
        if self.remaining == 0 {
            return None;
        }
        debug_assert!(!self.current.is_nil(), "iterator walked onto sentinel");

        // SAFETY: `current` is a valid non-nil node with an initialized value.
        // The lifetime 'a ties this iterator to the tree borrow,
        // so the tree cannot be mutated or dropped while
        // this iterator is alive. Each node is visited exactly once,
        // so no two mutable references to the same node can coexist.
        let value = unsafe { (*self.current.as_ptr()).value.assume_init_mut() };
        self.remaining -= 1;
        self.current = rb_successor(self.current, self.head);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<V> ExactSizeIterator for RbTreeIterMut<'_, V> {}
impl<'a, V> FusedIterator for RbTreeIterMut<'a, V> {}

impl<'a, V, A: Allocator, C: Sized, const UNIQUE: bool> IntoIterator
    for &'a RbTree<V, A, C, UNIQUE>
{
    type Item = &'a V;
    type IntoIter = RbTreeIter<'a, V>;

    fn into_iter(self) -> RbTreeIter<'a, V> {
        self.iter()
    }
}

impl<'a, V, A: Allocator, C: Sized, const UNIQUE: bool> IntoIterator
    for &'a mut RbTree<V, A, C, UNIQUE>
{
    type Item = &'a mut V;
    type IntoIter = RbTreeIterMut<'a, V>;

    fn into_iter(self) -> RbTreeIterMut<'a, V> {
        self.iter_mut()
    }
}

struct RotationPair<V> {
    /// Rotate toward the uncle
    toward: fn(NodePtr<V>, NodePtr<V>),
    /// Rotate away from the uncle
    away: fn(NodePtr<V>, NodePtr<V>),
}

/// Left rotation around `node`. `node`'s right child takes its place
///```text
///   Before:          After:
///
///     node           right
///     /  \           /   \
///    A   right     node   C
///        /  \      /  \
///       B    C    A    B
/// ```
fn rb_rotate_left<V>(mut node: NodePtr<V>, head: NodePtr<V>) {
    let mut right = node.right();

    node.set_right(right.left());
    if !right.left().is_nil() {
        right.left().set_parent(node);
    }

    unsafe { node.replace_in_parent(right, head) };
    right.set_left(node);
    node.set_parent(right);
}

/// Right rotation around `node`. `node`'s left child takes its place
///```text
///   Before:       After:
///
///       node      left
///       /  \      /  \
///     left  C    A   node
///     /  \           /  \
///    A    B         B    C
/// ```
fn rb_rotate_right<V>(mut node: NodePtr<V>, head: NodePtr<V>) {
    let mut left = node.left();

    node.set_left(left.right());
    if !left.right().is_nil() {
        left.right().set_parent(node);
    }

    unsafe { node.replace_in_parent(left, head) };

    left.set_right(node);
    node.set_parent(left);
}

fn rb_fixup_side<V>(node: &mut NodePtr<V>, head: NodePtr<V>, ops: RotationPair<V>) {
    let mut parent = node.parent();
    let mut grandparent = parent.parent();
    let mut uncle = if parent.is_left_child() {
        grandparent.right()
    } else {
        grandparent.left()
    };

    if uncle.is_red() {
        parent.set_black();
        uncle.set_black();
        grandparent.set_red();
        *node = grandparent;
    } else {
        let node_is_inner = if parent.is_left_child() {
            node.is_right_child()
        } else {
            node.is_left_child()
        };

        if node_is_inner {
            *node = parent;
            (ops.toward)(*node, head);
        }
        let mut new_parent = node.parent();
        let mut new_grandparent = new_parent.parent();
        new_parent.set_black();
        new_grandparent.set_red();
        (ops.away)(new_grandparent, head);
    }
}

fn rb_insert_fixup<V>(mut node: NodePtr<V>, head: NodePtr<V>) {
    while node.parent().is_red() {
        if node.parent().is_left_child() {
            rb_fixup_side(
                &mut node,
                head,
                RotationPair {
                    toward: rb_rotate_left,
                    away: rb_rotate_right,
                },
            );
        } else {
            rb_fixup_side(
                &mut node,
                head,
                RotationPair {
                    toward: rb_rotate_right,
                    away: rb_rotate_left,
                },
            );
        }
    }

    head.parent().set_black();
}

fn rb_erase_fixup<V>(mut fixnode: NodePtr<V>, mut fixparent: NodePtr<V>, head: NodePtr<V>) {
    while fixnode != head.parent() && !fixnode.is_red() {
        if fixnode == fixparent.left() {
            rb_erase_fixup_side::<V, true>(&mut fixnode, &mut fixparent, head);
        } else {
            rb_erase_fixup_side::<V, false>(&mut fixnode, &mut fixparent, head);
        }
    }
    fixnode.set_black();
}

fn rb_erase_fixup_side<V, const LEFT: bool>(
    fixnode: &mut NodePtr<V>,
    fixparent: &mut NodePtr<V>,
    head: NodePtr<V>,
) {
    let sibling = |fp: NodePtr<V>| if LEFT { fp.right() } else { fp.left() };
    let outer_child = |s: NodePtr<V>| if LEFT { s.right() } else { s.left() };
    let inner_child = |s: NodePtr<V>| if LEFT { s.left() } else { s.right() };
    let rotate_up = if LEFT {
        rb_rotate_left::<V>
    } else {
        rb_rotate_right::<V>
    };
    let rotate_down = if LEFT {
        rb_rotate_right::<V>
    } else {
        rb_rotate_left::<V>
    };

    let mut sib = sibling(*fixparent);

    // Case 1, sib red: rotate to get a black sib, fall through
    if sib.is_red() {
        sib.set_black();
        fixparent.set_red();
        rotate_up(*fixparent, head);
        sib = sibling(*fixparent);
    }

    if sib.is_nil() {
        *fixnode = *fixparent;
        *fixparent = fixnode.parent();
        return;
    }

    if !outer_child(sib).is_red() && !inner_child(sib).is_red() {
        // Case 2, both nephews black: recolour, propagate extra black upward
        sib.set_red();
        *fixnode = *fixparent;
        *fixparent = fixnode.parent();
    } else {
        if !outer_child(sib).is_red() {
            // Case 3, outer black, inner red: rotate at sib, outer becomes red
            inner_child(sib).set_black();
            sib.set_red();
            rotate_down(sib, head);
            sib = sibling(*fixparent);
        }
        // Case 4, outer nephew red: rotate at parent and recolour
        unsafe { sib.get_mut().color = fixparent.get().color };
        fixparent.set_black();
        outer_child(sib).set_black();
        rotate_up(*fixparent, head);
        *fixnode = head.parent();
    }
}

fn rb_successor<V>(node: NodePtr<V>, head: NodePtr<V>) -> NodePtr<V> {
    let right = node.right();
    if !right.is_nil() {
        right.leftmost()
    } else {
        let mut child = node;
        let mut parent = node.parent();
        while parent != head && child.is_right_child() {
            child = parent;
            parent = parent.parent();
        }
        parent
    }
}

fn rb_find_by<V>(head: NodePtr<V>, mut f: impl FnMut(&V) -> Ordering) -> Option<NodePtr<V>> {
    let mut node = head.parent();
    loop {
        if node.is_nil() {
            return None;
        }
        let val = unsafe { (*node.as_ptr()).value.assume_init_ref() };
        match f(val) {
            Ordering::Equal => return Some(node),
            Ordering::Less => node = node.right(),
            Ordering::Greater => node = node.left(),
        }
    }
}

fn rb_find_multi_start_by<V>(head: NodePtr<V>, mut f: impl FnMut(&V) -> Ordering) -> NodePtr<V> {
    let mut node = head.parent();
    let mut start = head;
    while !node.is_nil() {
        let val = unsafe { (*node.as_ptr()).value.assume_init_ref() };
        match f(val) {
            Ordering::Equal => {
                start = node;
                node = node.left();
            }
            Ordering::Less => node = node.right(),
            Ordering::Greater => node = node.left(),
        }
    }
    start
}
