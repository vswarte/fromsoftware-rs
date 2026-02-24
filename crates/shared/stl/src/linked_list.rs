use crate::allocator::*;
use std::{mem::MaybeUninit, ptr::NonNull};

#[repr(C)]
pub struct List<T, A: Sized> {
    #[cfg(not(feature = "msvc2012"))]
    allocator: A,
    head: NonNull<Node<T>>,
    length: usize,
    #[cfg(feature = "msvc2012")]
    allocator: A,
}

#[repr(C)]
struct Node<T> {
    next: NonNull<Node<T>>,
    previous: NonNull<Node<T>>,
    value: MaybeUninit<T>,
}

impl<T, A: crate::Allocator> List<T, A> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut length = self.length;
        let mut current = unsafe { self.head.as_ref() };

        std::iter::from_fn(move || {
            // Do this first to ensure we never deal with the sentinel head node.
            current = unsafe { current.next.as_ref() };

            if length == 0 {
                None
            } else {
                debug_assert!(
                    !std::ptr::eq(current, self.head.as_ptr()),
                    "attempted to use sentinel node value"
                );

                length -= 1;
                // Safety: only the root node should have an uninitialized value.
                Some(unsafe { current.value.assume_init_ref() })
            }
        })
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_back(&mut self, value: T) {
        let alloc = self.allocator.allocate::<Node<T>>();
        let new = NonNull::new(alloc).expect("allocator returned null");

        let mut head = self.head;
        let mut tail = unsafe { head.as_ref() }.previous;

        unsafe {
            std::ptr::write(
                new.as_ptr(),
                Node {
                    next: head,
                    previous: tail,
                    value: MaybeUninit::new(value),
                },
            );

            (tail.as_mut()).next = new;
            (head.as_mut()).previous = new;
        }

        // Safety: the user is probably more likely to run out of RAM before this :kekw:.
        self.length.checked_add(1).expect("list length overflow");
    }

    // TODO: insert that adds node after another

    pub fn pop_front(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let node = unsafe { self.head.as_ref() }.next;
        Some(unsafe { self.detach_node(node) })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }

        let node = unsafe { self.head.as_ref() }.previous;
        Some(unsafe { self.detach_node(node) })
    }

    /// # Safety
    ///
    /// `node` must be a node and to belong this list and not be the sentinel node.
    unsafe fn detach_node(&mut self, node: NonNull<Node<T>>) -> T {
        debug_assert!(node != self.head, "attempted to remove sentinel node");

        let node = node.as_ptr();
        let prev = unsafe { (*node).previous };
        let next = unsafe { (*node).previous };

        unsafe {
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).previous = prev;
        }

        self.length.checked_sub(1).expect("list length went below 0");

        let value = unsafe { (*node).value.assume_init_read() };
        let _ = self.allocator.deallocate_raw(node as _);

        value
    }
}
