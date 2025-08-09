use std::{
    collections::VecDeque,
    marker::PhantomData,
    ptr::{copy_nonoverlapping, NonNull},
};

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
        let mut pending = VecDeque::new();

        let start = unsafe { self.head.as_ref().parent };
        pending.push_back(start);

        // TODO: clean up this code
        std::iter::from_fn(move || {
            if self.size == 0 {
                None
            } else if let Some(mut entry) = pending.pop_front() {
                let entry = unsafe { entry.as_mut() };

                if unsafe { entry.left.as_ref() }.is_nil == 0 {
                    pending.push_back(entry.left);
                }

                if unsafe { entry.right.as_ref() }.is_nil == 0 {
                    pending.push_back(entry.right);
                }

                Some(&mut entry.value)
            } else {
                None
            }
        })
    }
}

#[repr(C)]
pub struct TreeNode<T> {
    left: NonNull<TreeNode<T>>,
    parent: NonNull<TreeNode<T>>,
    right: NonNull<TreeNode<T>>,
    black_red: u8,
    is_nil: u8,
    _pad1a: [u8; 6],
    value: T,
}

pub struct DLFixedVector<T, const N: usize>
where
    T: Sized,
{
    elements: [T; N],
    // TODO: fact-check this
    unk1: usize,
    count: usize,
}

impl<T, const N: usize> DLFixedVector<T, N>
where
    T: Sized,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements[0..self.count].iter()
    }
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
