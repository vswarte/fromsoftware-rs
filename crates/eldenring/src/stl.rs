use std::ptr::NonNull;

use crate::dlkr::{DLAllocatorBase, DLAllocatorRef};
use shared::OwnedPtr;

use cxx_stl::{list::CxxList, vec::CxxVec};

pub type DoublyLinkedList<T> = CxxList<T, DLAllocatorRef>;
pub type Vector<T> = CxxVec<T, DLAllocatorRef>;

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
