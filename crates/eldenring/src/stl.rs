use std::ptr::NonNull;

use crate::dlkr::DLAllocatorBase;
use shared::OwnedPtr;

#[repr(C)]
pub struct DoublyLinkedListNode<T> {
    pub next: NonNull<DoublyLinkedListNode<T>>,
    pub previous: NonNull<DoublyLinkedListNode<T>>,
    pub value: T,
}

#[repr(C)]
pub struct DoublyLinkedList<T> {
    allocator: usize,
    pub head: NonNull<DoublyLinkedListNode<T>>,
    pub count: u64,
}

impl<T> DoublyLinkedList<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut count = self.count;
        let mut current = unsafe { self.head.as_ref() };

        std::iter::from_fn(move || {
            current = unsafe { current.next.as_ref() };
            if count == 0 {
                None
            } else {
                count -= 1;
                Some(&current.value)
            }
        })
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[repr(C)]
pub struct Vector<T>
where
    T: Sized,
{
    allocator: NonNull<DLAllocatorBase>,
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<T> Vector<T>
where
    T: Sized,
{
    pub fn items(&self) -> &[T] {
        let Some(start) = self.begin else {
            return &mut [];
        };

        let end = self.end.unwrap();
        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        unsafe { std::slice::from_raw_parts(start.as_ptr(), count) }
    }

    pub fn items_mut(&mut self) -> &mut [T] {
        let Some(start) = self.begin else {
            return &mut [];
        };

        let end = self.end.unwrap();
        let count = (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>();

        unsafe { std::slice::from_raw_parts_mut(start.as_ptr(), count) }
    }

    pub fn len(&self) -> usize {
        let Some(end) = self.end else {
            return 0;
        };

        let Some(start) = self.begin else {
            return 0;
        };

        (end.as_ptr() as usize - start.as_ptr() as usize) / size_of::<T>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[repr(C)]
pub struct Tree<T> {
    allocator: usize,
    head: NonNull<TreeNode<T>>,
    size: usize,
}

impl<T> Tree<T> {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &mut T> {
        let mut current = unsafe {
            let head = self.head;
            let root = head.as_ref().parent;
            let min = Self::min_node(root);
            if min == head { None } else { Some(min) }
        };

        std::iter::from_fn(move || {
            let mut node = current?;
            unsafe {
                let node_ref = node.as_mut();
                let value_ref = &mut node_ref.value;

                // Advance current to next in-order node
                current = Self::next_inorder(node, self.head);

                Some(value_ref)
            }
        })
    }

    /// Finds the minimum (leftmost) node in a subtree.
    unsafe fn min_node(mut node: NonNull<TreeNode<T>>) -> NonNull<TreeNode<T>> {
        unsafe {
            while node.as_ref().is_nil == 0 && node.as_ref().left.as_ref().is_nil == 0 {
                node = node.as_ref().left;
            }
        }
        node
    }

    /// Returns the next in-order node from the given node.
    /// `head` is the sentinel node.
    unsafe fn next_inorder(
        mut node: NonNull<TreeNode<T>>,
        head: NonNull<TreeNode<T>>,
    ) -> Option<NonNull<TreeNode<T>>> {
        unsafe {
            if node.as_ref().right.as_ref().is_nil == 0 {
                // Go to the leftmost node in the right subtree
                Some(Self::min_node(node.as_ref().right))
            } else {
                // Walk up the tree until we find a node that is a left child
                loop {
                    let parent = node.as_ref().parent;
                    if parent == head || node != parent.as_ref().right {
                        return if parent == head { None } else { Some(parent) };
                    }
                    node = parent;
                }
            }
        }
    }
}

#[repr(C)]
pub struct TreeNode<T> {
    left: NonNull<TreeNode<T>>,
    parent: NonNull<TreeNode<T>>,
    right: NonNull<TreeNode<T>>,
    black_red: u8,
    is_nil: u8,
    value: T,
}

#[repr(C)]
pub struct ChainingTree<K, V> {
    base: Tree<Pair<K, ChainingMapBucketEntry<V>>>,
    buckets: OwnedPtr<ArrayWithHeader<ChainingMapBucketEntry<V>>>,
}

impl<K, V> ChainingTree<K, V> {
    pub fn len(&self) -> usize {
        self.base.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates over all key-value pairs, including all values in collision chains.
    /// Returns an iterator that yields `(&K, &V)` for each entry.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.base.iter().flat_map(|pair| {
            let key = &pair.key;
            pair.value.iter().map(move |value| (key, value))
        })
    }

    pub fn buckets(&self) -> &[ChainingMapBucketEntry<V>] {
        unsafe { self.buckets.as_slice() }
    }

    /// Iterates over all key-value pairs mutably, including all values in collision chains.
    /// Returns an iterator that yields `(&K, &mut V)` for each entry.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.base.iter().flat_map(|pair| {
            let key = &pair.key;
            pair.value.iter_mut().map(move |value| (key, value))
        })
    }

    /// Iterates over keys and their collision chain heads.
    /// Use this if you need to iterate collision chains separately.
    pub fn iter_chains(&self) -> impl Iterator<Item = (&K, &ChainingMapBucketEntry<V>)> {
        self.base.iter().map(|pair| (&pair.key, &pair.value))
    }
}

#[repr(C)]
pub struct ChainingMapBucketEntry<T> {
    pub data: T,
    pub next: Option<NonNull<ChainingMapBucketEntry<T>>>,
}

impl<T> ChainingMapBucketEntry<T> {
    /// Returns the number of entries in this collision chain.
    pub fn chain_len(&self) -> usize {
        self.iter().count()
    }

    /// Checks if this is the only entry in the chain (no next pointer).
    pub fn is_singleton(&self) -> bool {
        self.next.is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut current = Some(NonNull::from(self));
        std::iter::from_fn(move || {
            let node = current?;
            unsafe {
                let node_ref = node.as_ref();
                current = node_ref.next;
                Some(&node_ref.data)
            }
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let mut current = Some(NonNull::from(self));
        std::iter::from_fn(move || {
            let mut node = current?;
            unsafe {
                let node_ref = node.as_mut();
                current = node_ref.next;
                Some(&mut node_ref.data)
            }
        })
    }
}

#[repr(C)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}

#[repr(C)]
pub struct CSFixedList<T, const N: usize>
where
    T: Sized,
{
    vftable: usize,
    pub data: [CSFixedListEntry<T>; N],
    unk1: u32,
    unk2: u32,
    pub head_ptr: OwnedPtr<CSFixedListEntry<T>>,
    pub head: CSFixedListEntry<T>,
}

#[repr(C)]
pub struct CSFixedListEntry<T> {
    pub data: T,
    pub next: Option<NonNull<CSFixedListEntry<T>>>,
    pub previous: Option<NonNull<CSFixedListEntry<T>>>,
    index: usize,
}

/// Array with allocation metadata stored at negative offset.
///
/// [Header: -0x10] AllocationHeader
/// [Items: +0x00] Start of array elements
#[repr(C)]
pub struct ArrayWithHeader<T> {
    first_item: T,
}

impl<T> ArrayWithHeader<T> {
    /// Returns a slice of all items in the array.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid array with a properly initialized header.
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.first_item as *const T, self.len()) }
    }

    /// Returns a mutable slice of all items in the array.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid array with a properly initialized header.
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.first_item as *mut T, self.len()) }
    }

    /// Returns the allocation header stored before this array.
    ///
    /// # Safety
    ///
    /// The array must have a valid header at negative offset.
    pub unsafe fn header(&self) -> &AllocationHeader {
        unsafe {
            let header_ptr = (self as *const Self as *const u8)
                .sub(std::mem::size_of::<AllocationHeader>())
                as *const AllocationHeader;
            &*header_ptr
        }
    }

    /// Validates that the header's self-pointer matches its location.
    /// Used to detect memory corruption or invalid pointers.
    ///
    /// # Safety
    ///
    /// The array must have a valid header at negative offset.
    pub unsafe fn is_valid(&self) -> bool {
        unsafe { self.header().is_valid() }
    }

    /// Returns the number of items in the array.
    ///
    /// # Safety
    ///
    /// The array must have a valid header at negative offset.
    pub unsafe fn len(&self) -> usize {
        unsafe { self.header().count }
    }

    /// Returns true if the array is empty.
    ///
    /// # Safety
    ///
    /// The array must have a valid header at negative offset.
    pub unsafe fn is_empty(&self) -> bool {
        unsafe { self.len() == 0 }
    }
}

/// Allocation metadata stored before an `ArrayWithHeader`.
#[repr(C)]
pub struct AllocationHeader {
    /// Self-reference used for validation.
    /// Should always equal the address of this header.
    pub self_ptr: NonNull<AllocationHeader>,
    /// Number of items in the array.
    pub count: usize,
}

impl AllocationHeader {
    /// Checks if this header is valid by comparing self_ptr to actual location.
    pub fn is_valid(&self) -> bool {
        std::ptr::eq(self.self_ptr.as_ptr(), self)
    }
}
