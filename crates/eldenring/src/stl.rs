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
            if min == head {
                None
            } else {
                Some(min)
            }
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
        while node.as_ref().is_nil == 0 && node.as_ref().left.as_ref().is_nil == 0 {
            node = node.as_ref().left;
        }
        node
    }

    /// Returns the next in-order node from the given node.
    /// `head` is the sentinel node.
    unsafe fn next_inorder(
        mut node: NonNull<TreeNode<T>>,
        head: NonNull<TreeNode<T>>,
    ) -> Option<NonNull<TreeNode<T>>> {
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

#[repr(C)]
#[derive(PartialEq, Debug)]
/// Represents a C++ span, a collection that doesn't own its data.
pub struct DynamicSizeSpan<T> {
    data: *const T,
    length: usize,
}

impl<T> Default for DynamicSizeSpan<T> {
    fn default() -> Self {
        Self {
            data: 0x0 as _,
            length: 0,
        }
    }
}

impl<T> DynamicSizeSpan<T> {
    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    pub fn as_slice(&self) -> &[T] {
        if let Some(ptr) = unsafe { self.data.as_ref() } {
            unsafe { std::slice::from_raw_parts(ptr as _, self.length) }
        } else {
            &[]
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// # Safety
    ///
    /// Caller must ensure the memory is valid and of size `self.size`
    pub unsafe fn as_mut(&mut self) -> &mut [T] {
        if let Some(ptr) = self.data.as_ref() {
            unsafe { std::slice::from_raw_parts_mut(ptr as *const _ as *mut _, self.length) }
        } else {
            &mut []
        }
    }

    /// # Safety
    ///
    /// Caller must ensure the memory is valid and of size `self.size` and that access is
    /// exclusive.
    pub unsafe fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.as_mut().iter_mut()
    }

    /// # Safety
    ///
    /// Caller must ensure that the data argument is populated with a valid non-null pointer, that
    /// type T is complete and properly sized and the size param represents the proper count for
    /// the collection.
    pub unsafe fn from_raw_parts(data: *const T, length: usize) -> Self {
        Self { data, length }
    }

    pub const fn from_static_slice(v: &'static [T]) -> Self {
        Self {
            data: v.as_ptr(),
            length: v.len(),
        }
    }

    pub const fn empty() -> Self {
        Self { data: 0x0 as _, length: 0 }
    }
}
