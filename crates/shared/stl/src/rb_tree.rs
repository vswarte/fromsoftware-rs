use crate::{Pair, allocator::*};
use std::{
    iter::FusedIterator,
    mem::MaybeUninit,
    ops::{Bound, Deref, DerefMut},
    ptr::NonNull,
};

/// Comparator trait for use in MSVC `std::tree` [`RbTree`]
pub trait TreeComparator<V> {
    type Key: ?Sized;
    /// Returns `true` if `a < b`.
    fn lt(&self, a: &V, b: &V) -> bool;
    /// Returns `true` if `key < val`.
    fn lt_key_val(&self, key: &Self::Key, val: &V) -> bool;
    /// Returns `true` if `val < key`.
    fn lt_val_key(&self, val: &V, key: &Self::Key) -> bool;
    /// Returns `true` if `val > key`, i.e. `key < val`.
    #[inline]
    fn gt_val_key(&self, val: &V, key: &Self::Key) -> bool {
        self.lt_key_val(key, val)
    }
    /// Returns `true` if `val >= key`, i.e. `!(val < key)`.
    #[inline]
    fn gte_val_key(&self, val: &V, key: &Self::Key) -> bool {
        !self.lt_val_key(val, key)
    }
    /// Returns `true` if `val <= key`, i.e. `!(key < val)`.
    #[inline]
    fn lte_val_key(&self, val: &V, key: &Self::Key) -> bool {
        !self.lt_key_val(key, val)
    }
    /// Returns `true` if `key >= val`, i.e. `!(key < val)`.
    #[inline]
    fn gte_key_val(&self, key: &Self::Key, val: &V) -> bool {
        !self.lt_key_val(key, val)
    }
    /// Returns `true` if `key == val`, i.e. `!(key < val) && !(val < key)`.
    #[inline]
    fn eq_key(&self, key: &Self::Key, val: &V) -> bool {
        !self.lt_key_val(key, val) && !self.lt_val_key(val, key)
    }
}

/// Default [`RbTree`] comparator, equivalent to MSVC [`std::less<V>`]
///
/// [`std::less<V>`]: https://en.cppreference.com/w/cpp/utility/functional/less.html
#[derive(Default)]
pub struct Less;

impl<V: Ord> TreeComparator<V> for Less {
    type Key = V;

    #[inline]
    fn lt(&self, a: &V, b: &V) -> bool {
        a < b
    }
    #[inline]
    fn lt_key_val(&self, key: &V, val: &V) -> bool {
        key < val
    }
    #[inline]
    fn lt_val_key(&self, val: &V, key: &V) -> bool {
        val < key
    }
}

/// Comparator for `Map<K, V>` that orders by key only,
/// equivalent to MSVC [`std::less<K>`] applied to [`std::pair::first`]
///
/// [`std::less<K>`]: https://en.cppreference.com/w/cpp/utility/functional/less.html
/// [`std::pair::first`]: https://en.cppreference.com/w/cpp/utility/pair.html
#[derive(Default)]
pub struct KeyLess;

impl<K: Ord, V> TreeComparator<Pair<K, V>> for KeyLess {
    type Key = K;

    #[inline]
    fn lt(&self, a: &Pair<K, V>, b: &Pair<K, V>) -> bool {
        a.first < b.first
    }
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
enum RbColor {
    Red = 0,
    Black = 1,
}

#[repr(C)]
struct RbNode<V> {
    left: NodePtr<V>,
    parent: NodePtr<V>,
    right: NodePtr<V>,
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
    /// # Safety
    ///
    /// No other reference to the node this pointer points to may exist
    /// at the same time. Since `NodePtr` is `Copy`, the borrow checker
    /// cannot enforce this, the caller must guarantee it
    #[inline]
    unsafe fn get_mut(&mut self) -> &mut RbNode<V> {
        unsafe { self.0.as_mut() }
    }
    #[inline]
    fn as_ptr(&self) -> *mut RbNode<V> {
        self.0.as_ptr()
    }
    #[inline]
    fn left(&self) -> NodePtr<V> {
        self.get().left
    }
    #[inline]
    fn right(&self) -> NodePtr<V> {
        self.get().right
    }
    #[inline]
    fn parent(&self) -> NodePtr<V> {
        self.get().parent
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
        unsafe { self.get_mut().left = n };
    }
    #[inline]
    fn set_right(&mut self, n: NodePtr<V>) {
        unsafe { self.get_mut().right = n };
    }
    #[inline]
    fn set_parent(&mut self, n: NodePtr<V>) {
        unsafe { self.get_mut().parent = n };
    }
    #[inline]
    fn is_left_child(&self) -> bool {
        *self == self.parent().left()
    }
    #[inline]
    fn is_right_child(&self) -> bool {
        *self == self.parent().right()
    }
    /// # Safety
    ///
    /// Must not be called on the head/sentinel node
    #[inline]
    unsafe fn value<'a>(self) -> &'a V {
        unsafe { &*(*self.as_ptr()).value.as_ptr() }
    }
    /// # Safety
    ///
    /// Must not be called on the head/sentinel node. No other reference to
    /// this node's value may exist at the same time
    #[inline]
    unsafe fn value_mut<'a>(self) -> &'a mut V {
        unsafe { &mut *(*self.as_ptr()).value.as_mut_ptr() }
    }
    /// # Safety
    ///
    /// Must not be called on the head/sentinel node. The node must not be
    /// accessed again after this call
    #[inline]
    unsafe fn value_read(self) -> V {
        unsafe { (*self.as_ptr()).value.assume_init_read() }
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
    /// `replacement` must be a valid node
    unsafe fn replace_in_parent(&mut self, mut replacement: NodePtr<V>, head: NodePtr<V>) {
        if !replacement.is_nil() {
            replacement.set_parent(self.parent());
        }
        unsafe {
            if self.parent() == head {
                self.parent().get_mut().parent = replacement;
            } else if self.is_left_child() {
                self.parent().get_mut().left = replacement;
            } else {
                self.parent().get_mut().right = replacement;
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

/// Internal MSVC C++ Red-Black Tree implementation.
///
/// Prefer the [`Map`], [`Set`], [`MultiMap`], [`MultiSet`] newtypes over using
/// this type directly.
///
/// # References
///
/// - [MSVC STL source - `xtree`]
/// - [Raymond Chen's breakdown of `xtree`]
///
/// [MSVC STL source - `xtree`]: https://github.com/microsoft/STL/blob/main/stl/inc/xtree
/// [Raymond Chen's breakdown of `xtree`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
#[repr(C)]
pub struct RbTree<V, A: Allocator, C: Sized, const UNIQUE: bool = true> {
    comparator: C,
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    pub allocator: A,
    head: NodePtr<V>,
    size: usize,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    pub allocator: A,
}

/// Implementation of MSVC C++ `std::map`
///
/// # References
///
/// - [cppreference - `std::map`]
/// - [MSVC STL source - `map`]
/// - [Raymond Chen's breakdown of `std::map`]
///
/// [cppreference - `std::map`]: https://en.cppreference.com/w/cpp/container/map.html
/// [MSVC STL source - `map`]: https://github.com/microsoft/STL/blob/main/stl/inc/map
/// [Raymond Chen's breakdown of `std::map`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
#[repr(transparent)]
pub struct Map<K, V, A: Allocator, C: TreeComparator<Pair<K, V>> = KeyLess>(
    RbTree<Pair<K, V>, A, C, true>,
);
/// Implementation of MSVC C++ `std::set`
///
/// # References
///
/// - [cppreference - `std::set`]
/// - [MSVC STL source - `set`]
/// - [Raymond Chen's breakdown of `std::set`]
///
/// [cppreference - `std::set`]: https://en.cppreference.com/w/cpp/container/set.html
/// [MSVC STL source - `set`]: https://github.com/microsoft/STL/blob/main/stl/inc/set
/// [Raymond Chen's breakdown of `std::set`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
#[repr(transparent)]
pub struct Set<K, A: Allocator, C: TreeComparator<K> = Less>(RbTree<K, A, C, true>);
/// Implementation of MSVC C++ `std::multimap`
///
/// # References
///
/// - [cppreference - `std::multimap`]
/// - [MSVC STL source - `multimap`]
/// - [Raymond Chen's breakdown of `std::multimap`]
///
/// [cppreference - `std::multimap`]: https://en.cppreference.com/w/cpp/container/multimap.html
/// [MSVC STL source - `multimap`]: https://github.com/microsoft/STL/blob/main/stl/inc/map
/// [Raymond Chen's breakdown of `std::multimap`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
#[repr(transparent)]
pub struct MultiMap<K, V, A: Allocator, C: TreeComparator<Pair<K, V>> = KeyLess>(
    RbTree<Pair<K, V>, A, C, false>,
);
/// Implementation of MSVC C++ `std::multiset`
///
/// # References
///
/// - [cppreference - `std::multiset`]
/// - [MSVC STL source - `multiset`]
/// - [Raymond Chen's breakdown of `std::multiset`]
///
/// [cppreference - `std::multiset`]: https://en.cppreference.com/w/cpp/container/multiset.html
/// [MSVC STL source - `multiset`]: https://github.com/microsoft/STL/blob/main/stl/inc/set
/// [Raymond Chen's breakdown of `std::multiset`]: https://devblogs.microsoft.com/oldnewthing/20230807-00/?p=108562
#[repr(transparent)]
pub struct MultiSet<K, A: Allocator, C: TreeComparator<K> = Less>(RbTree<K, A, C, false>);

impl<V, A: Allocator, C: TreeComparator<V> + Default, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    /// Creates an empty tree using the provided allocator and a default-constructed comparator
    pub fn new_in(mut allocator: A) -> Self {
        let head = Self::alloc_sentinel(&mut allocator);
        Self {
            comparator: C::default(),
            allocator,
            head,
            size: 0,
        }
    }
}

impl<V, A: Allocator, C: Sized, const UNIQUE: bool> Drop for RbTree<V, A, C, UNIQUE> {
    fn drop(&mut self) {
        let root = self.head.parent();
        if !root.is_nil() {
            unsafe { self.drop_subtree(root) };
        }
        // Free the sentinel itself (its value field is never initialized)
        unsafe { self.allocator.deallocate_raw(self.head.as_ptr() as _) };
    }
}

impl<V, A: Allocator, C: Sized, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    /// Creates an empty tree using the provided allocator and comparator
    pub fn new_in_with(mut allocator: A, comparator: C) -> Self {
        let head = Self::alloc_sentinel(&mut allocator);
        Self {
            comparator,
            allocator,
            head,
            size: 0,
        }
    }

    fn alloc_sentinel(allocator: &mut A) -> NodePtr<V> {
        let node: NodePtr<V> = NodePtr(allocator.allocate::<RbNode<V>>());
        unsafe {
            std::ptr::write(
                node.as_ptr(),
                RbNode {
                    left: node,
                    parent: node,
                    right: node,
                    color: RbColor::Black,
                    is_nil: true,
                    value: MaybeUninit::uninit(),
                },
            );
        }
        node
    }

    unsafe fn drop_subtree(&mut self, root: NodePtr<V>) {
        if root.is_nil() {
            return;
        }

        // "Unbalance" the tree by repeatedly rotating left children up.
        // Then drop all right nodes
        let mut current = root;
        while !current.is_nil() {
            if !current.left().is_nil() {
                // Right-rotate: bring left child up, push current down-right
                let mut left = current.left();
                current.set_left(left.right());
                left.set_right(current);
                current = left;
            } else {
                // No left child -> drop value, free node, advance right
                let next = current.right();
                unsafe {
                    std::ptr::drop_in_place((*current.as_ptr()).value.as_mut_ptr());
                    self.allocator.deallocate_raw(current.as_ptr() as _);
                }
                current = next;
            }
        }
    }
}

impl<V, A: Allocator, C: TreeComparator<V>, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    fn bound_node<const LEFT: bool>(&self, bound: Bound<&C::Key>) -> Option<NodePtr<V>> {
        let v = match bound {
            Bound::Unbounded => {
                let n = if LEFT {
                    self.head.left()
                } else {
                    self.head.right()
                };
                return (!n.is_nil()).then_some(n);
            }
            Bound::Included(v) | Bound::Excluded(v) => v,
        };
        let included = matches!(bound, Bound::Included(_));
        let mut node = self.head.get().parent;
        let mut result = None;
        loop {
            if node.is_nil() {
                return result;
            }
            let nv = unsafe { node.value() };
            let go = match (LEFT, included) {
                (true, true) => self.comparator.gte_val_key(nv, v),
                (true, false) => self.comparator.gt_val_key(nv, v),
                (false, true) => self.comparator.lte_val_key(nv, v),
                (false, false) => self.comparator.lt_val_key(nv, v),
            };
            if go {
                result = Some(node);
                node = if LEFT { node.left() } else { node.right() };
            } else {
                node = if LEFT { node.right() } else { node.left() };
            }
        }
    }

    /// Returns a reference to the first element satisfying the bound.
    ///
    /// - `Included(k)` -> first element with key `>= k`
    /// - `Excluded(k)` -> first element with key `> k`
    /// - `Unbounded`   -> first element in the tree
    pub fn lower_bound(&self, bound: Bound<&C::Key>) -> Option<&V> {
        self.bound_node::<true>(bound).map(|n| unsafe { n.value() })
    }

    /// Returns a mutable reference to the first element satisfying the bound.
    ///
    /// - `Included(k)` -> first element with key `>= k`
    /// - `Excluded(k)` -> first element with key `> k`
    /// - `Unbounded`   -> first element in the tree
    pub fn lower_bound_mut(&mut self, bound: Bound<&C::Key>) -> Option<&mut V> {
        self.bound_node::<true>(bound)
            .map(|n| unsafe { n.value_mut() })
    }

    /// Returns a reference to the last element satisfying the bound.
    ///
    /// - `Included(k)` -> last element with key `<= k`
    /// - `Excluded(k)` -> last element with key `< k`
    /// - `Unbounded`   -> last element in the tree
    pub fn upper_bound(&self, bound: Bound<&C::Key>) -> Option<&V> {
        self.bound_node::<false>(bound)
            .map(|n| unsafe { n.value() })
    }

    /// Returns a mutable reference to the last element satisfying the bound.
    ///
    /// - `Included(k)` -> last element with key `<= k`
    /// - `Excluded(k)` -> last element with key `< k`
    /// - `Unbounded`   -> last element in the tree
    pub fn upper_bound_mut(&mut self, bound: Bound<&C::Key>) -> Option<&mut V> {
        self.bound_node::<false>(bound)
            .map(|n| unsafe { n.value_mut() })
    }

    /// Returns `Ok(new_node)` on insertion, or `Err((existing_node, value))`
    /// on collision when `UNIQUE = true`
    fn _try_insert(&mut self, value: V) -> Result<NodePtr<V>, (NodePtr<V>, V)> {
        let mut head = self.head;
        let mut parent = head;
        let mut node = head.parent();
        let mut insert_left = true;

        while !node.is_nil() {
            parent = node;
            let node_val = unsafe { node.value() };

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
                return Err((node, value));
            }
        }

        let new_node = NodePtr(self.allocator.allocate::<RbNode<V>>().cast());
        unsafe {
            std::ptr::write(
                new_node.as_ptr(),
                RbNode {
                    left: head,
                    parent,
                    right: head,
                    color: RbColor::Red,
                    is_nil: false,
                    value: MaybeUninit::new(value),
                },
            );
        }

        if parent == head {
            unsafe {
                head.get_mut().parent = new_node;
                head.get_mut().left = new_node;
                head.get_mut().right = new_node;
            }
        } else if insert_left {
            parent.set_left(new_node);
            if parent == head.left() {
                unsafe { head.get_mut().left = new_node };
            }
        } else {
            parent.set_right(new_node);
            if parent == head.right() {
                unsafe { head.get_mut().right = new_node };
            }
        }

        self.size = self.size.checked_add(1).expect("tree size overflow");
        rb_insert_fixup(new_node, head);
        // Safety: only the root node should have an uninitialized value
        Ok(new_node)
    }

    /// Removes the element matching `key`, returning it if present
    pub fn remove(&mut self, key: &C::Key) -> Option<V> {
        let node = self.find_node(key)?;
        Some(unsafe { self.extract_node(node) })
    }

    pub fn pop_min(&mut self) -> Option<V> {
        (self.size > 0).then(|| unsafe { self.extract_node(self.head.left()) })
    }

    pub fn pop_max(&mut self) -> Option<V> {
        (self.size > 0).then(|| unsafe { self.extract_node(self.head.right()) })
    }

    fn find_node(&self, key: &C::Key) -> Option<NodePtr<V>> {
        self.bound_node::<true>(Bound::Included(key))
            .filter(|n| self.comparator.eq_key(key, unsafe { n.value() }))
    }

    /// Port of MSVC `_Tree_val::_Extract` + `_Tree_node::_Freenode`
    ///
    /// # Safety
    ///
    /// `erased` must be a live non-sentinel node belonging to this tree
    unsafe fn extract_node(&mut self, mut erased: NodePtr<V>) -> V {
        let mut head = self.head;

        let (fixnode, fixparent) = unsafe {
            if erased.left().is_nil() || erased.right().is_nil() {
                let fix = if erased.left().is_nil() {
                    erased.right()
                } else {
                    erased.left()
                };
                erased.replace_in_parent(fix, head);
                (fix, erased.parent())
            } else {
                let mut succ = rb_successor(erased, head);
                let mut fix = succ.right();

                let fixparent = if succ == erased.right() {
                    succ
                } else {
                    let mut sp = succ.parent();
                    if !fix.is_nil() {
                        fix.set_parent(sp);
                    }
                    sp.get_mut().left = fix;
                    succ.set_right(erased.right());
                    erased.right().set_parent(succ);
                    sp
                };

                succ.set_left(erased.left());
                erased.left().set_parent(succ);
                erased.replace_in_parent(succ, head);

                let (ec, sc) = (erased.get().color, succ.get().color);
                succ.get_mut().color = ec;
                erased.get_mut().color = sc;

                (fix, fixparent)
            }
        };

        if head.parent() == erased {
            unsafe { head.get_mut().parent = fixnode };
        }
        if head.left() == erased {
            let m = if fixnode.is_nil() {
                fixparent
            } else {
                fixnode.leftmost()
            };
            unsafe { head.get_mut().left = m };
        }
        if head.right() == erased {
            let m = if fixnode.is_nil() {
                fixparent
            } else {
                fixnode.rightmost()
            };
            unsafe { head.get_mut().right = m };
        }

        if erased.get().color == RbColor::Black {
            rb_erase_fixup(fixnode, fixparent, head);
        }

        self.size = self.size.checked_sub(1).expect("tree size went below 0");
        let value = unsafe { erased.value_read() };
        unsafe { self.allocator.deallocate_raw(erased.as_ptr() as _) };
        value
    }
}

impl<V, A: Allocator, C: Sized, const UNIQUE: bool> RbTree<V, A, C, UNIQUE> {
    /// Inorder iterator, yields values in ascending comparator order
    pub fn iter(&self) -> RbTreeIter<'_, V> {
        let head = self.head;
        RbTreeIter {
            head,
            current: head.left(),
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> RbTreeIterMut<'_, V> {
        let head = self.head;
        RbTreeIterMut {
            head,
            current: head.left(),
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
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

// Set API
impl<V, A: Allocator, C: TreeComparator<V>> Set<V, A, C> {
    pub fn contains(&self, key: &C::Key) -> bool {
        self.find_node(key).is_some()
    }

    /// Returns a reference to the element matching `key` or `None`
    pub fn find(&self, key: &C::Key) -> Option<&V> {
        self.find_node(key).map(|n| unsafe { n.value() })
    }

    /// Returns a mutable reference to the element matching `key` or `None`
    pub fn find_mut(&mut self, key: &C::Key) -> Option<&mut V> {
        self.find_node(key).map(|n| unsafe { n.value_mut() })
    }

    /// Adds a value to the set.
    ///
    /// Returns `true` if the value was newly inserted, `false` if it already existed
    pub fn insert(&mut self, value: V) -> bool {
        self._try_insert(value).is_ok()
    }

    /// Returns the existing element if already present, otherwise inserts and
    /// returns a reference to the new value
    pub fn get_or_insert(&mut self, value: V) -> &V {
        let node = match self._try_insert(value) {
            Ok(n) | Err((n, _)) => n,
        };
        unsafe { node.value() }
    }
}

// MultiSet API
impl<V, A: Allocator, C: TreeComparator<V>> MultiSet<V, A, C> {
    pub fn contains(&self, key: &C::Key) -> bool {
        self.find_node(key).is_some()
    }

    /// Returns an iterator over all elements matching `key`
    pub fn find(&self, key: &C::Key) -> impl Iterator<Item = &V> {
        let head = self.head;
        let start = self
            .bound_node::<true>(Bound::Included(key))
            .unwrap_or(head);
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(|v| self.comparator.eq_key(key, v))
    }

    /// Returns an iterator over mutable references to all elements matching `key`
    pub fn find_mut(&mut self, key: &C::Key) -> impl Iterator<Item = &mut V> {
        let head = self.head;
        let start = self
            .bound_node::<true>(Bound::Included(key))
            .unwrap_or(head);
        RbTreeIterMut {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(|v| self.comparator.eq_key(key, v))
    }

    /// Inserts a value (duplicates are allowed)
    pub fn insert(&mut self, value: V) {
        // UNIQUE=false so insert_inner always returns Ok
        let _ = self._try_insert(value);
    }

    /// Removes all elements matching `key`, returns the count removed
    pub fn remove_all(&mut self, key: &C::Key) -> usize {
        std::iter::from_fn(|| self.remove(key)).count()
    }
}

// Map API
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>, Key = K>> Map<K, V, A, C> {
    pub fn contains(&self, key: &K) -> bool {
        self.find_node(key).is_some()
    }

    /// Returns a reference to the value associated with `key`, or `None`
    pub fn find(&self, key: &K) -> Option<&V> {
        self.find_node(key).map(|n| unsafe { &n.value().second })
    }

    /// Returns a mutable reference to the value associated with `key`, or `None`
    pub fn find_mut(&mut self, key: &K) -> Option<&mut V> {
        self.find_node(key)
            .map(|n| unsafe { &mut n.value_mut().second })
    }

    /// Inserts a key-value pair.
    ///
    /// If the key was not present, returns `None`.
    /// If the key was already present, replaces the value and returns the old one
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self._try_insert(Pair {
            first: key,
            second: value,
        }) {
            Ok(_) => None,
            Err((node, new_pair)) => {
                let old = unsafe { std::ptr::read(&node.value_mut().second) };
                unsafe { std::ptr::write(&mut node.value_mut().second, new_pair.second) };
                Some(old)
            }
        }
    }

    /// Tries to insert a key-value pair.
    ///
    /// Returns `Ok(&mut V)` if the key was newly inserted \
    /// Returns `Err(&mut V)` with a reference to the existing value if the key was already present
    pub fn try_insert(&mut self, key: K, value: V) -> Result<&mut V, &mut V> {
        match self._try_insert(Pair {
            first: key,
            second: value,
        }) {
            Ok(node) => Ok(unsafe { &mut node.value_mut().second }),
            Err((node, _)) => Err(unsafe { &mut node.value_mut().second }),
        }
    }
}

// MultiMap API
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>, Key = K>> MultiMap<K, V, A, C> {
    pub fn contains(&self, key: &K) -> bool {
        self.find_node(key).is_some()
    }

    /// Returns an iterator over all values associated with `key`
    pub fn find(&self, key: &K) -> impl Iterator<Item = &V> {
        let head = self.head;
        let start = self
            .bound_node::<true>(Bound::Included(key))
            .unwrap_or(head);
        let cmp = &self.comparator;
        RbTreeIter {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(|v| cmp.eq_key(key, v))
        .map(|p| &p.second)
    }

    /// Returns an iterator over mutable references to all values associated with `key`
    pub fn find_mut(&mut self, key: &K) -> impl Iterator<Item = &mut V> {
        let head = self.head;
        let start = self
            .bound_node::<true>(Bound::Included(key))
            .unwrap_or(head);
        let cmp = &self.comparator;
        RbTreeIterMut {
            head,
            current: start,
            remaining: self.size,
            _marker: std::marker::PhantomData,
        }
        .take_while(|v| cmp.eq_key(key, v))
        .map(|p| &mut p.second)
    }

    /// Inserts a key-value pair (duplicates are allowed)
    pub fn insert(&mut self, key: K, value: V) {
        let _ = self._try_insert(Pair {
            first: key,
            second: value,
        });
    }

    /// Removes all elements matching `key`, returns the count removed
    pub fn remove_all(&mut self, key: &K) -> usize {
        std::iter::from_fn(|| self.remove(key)).count()
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
        let value = unsafe { self.current.value() };
        self.remaining -= 1;
        self.current = rb_successor(self.current, self.head);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<V> ExactSizeIterator for RbTreeIter<'_, V> {}
impl<V> FusedIterator for RbTreeIter<'_, V> {}

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
        let value = unsafe { self.current.value_mut() };
        self.remaining -= 1;
        self.current = rb_successor(self.current, self.head);
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<V> ExactSizeIterator for RbTreeIterMut<'_, V> {}
impl<V> FusedIterator for RbTreeIterMut<'_, V> {}

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

impl<'a, K, A: Allocator, C: TreeComparator<K>> IntoIterator for &'a Set<K, A, C> {
    type Item = &'a K;
    type IntoIter = RbTreeIter<'a, K>;
    fn into_iter(self) -> RbTreeIter<'a, K> {
        self.0.iter()
    }
}
impl<'a, K, A: Allocator, C: TreeComparator<K>> IntoIterator for &'a mut Set<K, A, C> {
    type Item = &'a mut K;
    type IntoIter = RbTreeIterMut<'a, K>;
    fn into_iter(self) -> RbTreeIterMut<'a, K> {
        self.0.iter_mut()
    }
}

impl<'a, K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> IntoIterator for &'a Map<K, V, A, C> {
    type Item = &'a Pair<K, V>;
    type IntoIter = RbTreeIter<'a, Pair<K, V>>;
    fn into_iter(self) -> RbTreeIter<'a, Pair<K, V>> {
        self.0.iter()
    }
}
impl<'a, K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> IntoIterator
    for &'a mut Map<K, V, A, C>
{
    type Item = &'a mut Pair<K, V>;
    type IntoIter = RbTreeIterMut<'a, Pair<K, V>>;
    fn into_iter(self) -> RbTreeIterMut<'a, Pair<K, V>> {
        self.0.iter_mut()
    }
}

impl<'a, K, A: Allocator, C: TreeComparator<K>> IntoIterator for &'a MultiSet<K, A, C> {
    type Item = &'a K;
    type IntoIter = RbTreeIter<'a, K>;
    fn into_iter(self) -> RbTreeIter<'a, K> {
        self.0.iter()
    }
}
impl<'a, K, A: Allocator, C: TreeComparator<K>> IntoIterator for &'a mut MultiSet<K, A, C> {
    type Item = &'a mut K;
    type IntoIter = RbTreeIterMut<'a, K>;
    fn into_iter(self) -> RbTreeIterMut<'a, K> {
        self.0.iter_mut()
    }
}

impl<'a, K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> IntoIterator
    for &'a MultiMap<K, V, A, C>
{
    type Item = &'a Pair<K, V>;
    type IntoIter = RbTreeIter<'a, Pair<K, V>>;
    fn into_iter(self) -> RbTreeIter<'a, Pair<K, V>> {
        self.0.iter()
    }
}
impl<'a, K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> IntoIterator
    for &'a mut MultiMap<K, V, A, C>
{
    type Item = &'a mut Pair<K, V>;
    type IntoIter = RbTreeIterMut<'a, Pair<K, V>>;
    fn into_iter(self) -> RbTreeIterMut<'a, Pair<K, V>> {
        self.0.iter_mut()
    }
}

impl<K, A: Allocator, C: TreeComparator<K>> Deref for Set<K, A, C> {
    type Target = RbTree<K, A, C, true>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, A: Allocator, C: TreeComparator<K>> DerefMut for Set<K, A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> Deref for Map<K, V, A, C> {
    type Target = RbTree<Pair<K, V>, A, C, true>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> DerefMut for Map<K, V, A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, A: Allocator, C: TreeComparator<K>> Deref for MultiSet<K, A, C> {
    type Target = RbTree<K, A, C, false>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, A: Allocator, C: TreeComparator<K>> DerefMut for MultiSet<K, A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> Deref for MultiMap<K, V, A, C> {
    type Target = RbTree<Pair<K, V>, A, C, false>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>>> DerefMut for MultiMap<K, V, A, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, A: Allocator, C: TreeComparator<K> + Default> Set<K, A, C> {
    pub fn new_in(allocator: A) -> Self {
        Set(RbTree::new_in(allocator))
    }
}
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>> + Default> Map<K, V, A, C> {
    pub fn new_in(allocator: A) -> Self {
        Map(RbTree::new_in(allocator))
    }
}
impl<K, A: Allocator, C: TreeComparator<K> + Default> MultiSet<K, A, C> {
    pub fn new_in(allocator: A) -> Self {
        MultiSet(RbTree::new_in(allocator))
    }
}
impl<K, V, A: Allocator, C: TreeComparator<Pair<K, V>> + Default> MultiMap<K, V, A, C> {
    pub fn new_in(allocator: A) -> Self {
        MultiMap(RbTree::new_in(allocator))
    }
}

struct RotationPair<V> {
    /// Rotate toward the uncle
    toward: fn(NodePtr<V>, NodePtr<V>),
    /// Rotate away from the uncle
    away: fn(NodePtr<V>, NodePtr<V>),
}

/// Left rotation around `node`. `node`'s right child takes its place.
/// ```text
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

/// Right rotation around `node`. `node`'s left child takes its place.
/// ```text
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

fn rb_fixup_side<V>(node: NodePtr<V>, head: NodePtr<V>, ops: RotationPair<V>) -> NodePtr<V> {
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
        grandparent
    } else {
        let node_is_inner = if parent.is_left_child() {
            node.is_right_child()
        } else {
            node.is_left_child()
        };

        let (fix_node, mut new_parent) = if node_is_inner {
            (ops.toward)(parent, head);
            (parent, node)
        } else {
            (node, parent)
        };

        new_parent.set_black();
        new_parent.parent().set_red();
        (ops.away)(new_parent.parent(), head);
        fix_node
    }
}

fn rb_insert_fixup<V>(node: NodePtr<V>, head: NodePtr<V>) {
    let mut node = node;
    while node.parent().is_red() {
        node = if node.parent().is_left_child() {
            rb_fixup_side(
                node,
                head,
                RotationPair {
                    toward: rb_rotate_left,
                    away: rb_rotate_right,
                },
            )
        } else {
            rb_fixup_side(
                node,
                head,
                RotationPair {
                    toward: rb_rotate_right,
                    away: rb_rotate_left,
                },
            )
        };
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
