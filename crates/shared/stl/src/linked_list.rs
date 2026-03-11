use crate::allocator::*;
use std::{mem::MaybeUninit, ptr::NonNull};

#[repr(C)]
/// Implementation of MSVC C++ `std::list`.
///
/// # References
///
/// - [cppreference - `std::list`]
/// - [MSVC STL source - `list`]
/// - [Raymond Chen's breakdown of `std::list`]
///
/// [cppreference - `std::list`]: https://en.cppreference.com/w/cpp/container/list.html
/// [MSVC STL source - `list`]: https://github.com/microsoft/STL/blob/main/stl/inc/list
/// [Raymond Chen's breakdown of `std::list`]: https://devblogs.microsoft.com/oldnewthing/20230804-00/?p=108547
pub struct List<T, A: Allocator> {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    allocator: A,
    head: NonNull<Node<T>>,
    length: usize,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    allocator: A,
}

#[repr(C)]
struct Node<T> {
    next: NonNull<Node<T>>,
    previous: NonNull<Node<T>>,
    value: MaybeUninit<T>,
}

impl<T, A: Allocator> List<T, A> {
    /// Creates an empty list backed by `allocator`.
    ///
    /// Equivalent to `std::list<T>()` with a custom allocator
    pub fn new_in(mut allocator: A) -> Self {
        // Allocate the sentinel head node. Its value is never initialized
        let head = allocator.allocate::<Node<T>>();
        unsafe {
            std::ptr::write(
                head.as_ptr(),
                Node {
                    next: head,
                    previous: head,
                    value: MaybeUninit::uninit(),
                },
            );
        }
        Self {
            allocator,
            head,
            length: 0,
        }
    }

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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        let mut length = self.length;
        let mut current = unsafe { self.head.as_ref() }.next;
        let head = self.head;

        std::iter::from_fn(move || {
            if length == 0 {
                return None;
            }

            debug_assert!(current != head, "attempted to use sentinel node value");

            // Safety: only the root node should have an uninitialized value.
            let value = unsafe { (*current.as_ptr()).value.assume_init_mut() };
            length -= 1;
            current = unsafe { current.as_ref() }.next;
            Some(value)
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
        let new = self.allocator.allocate::<Node<T>>();

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
        self.length = self.length.checked_add(1).expect("list length overflow");
    }

    pub fn push_front(&mut self, value: T) {
        let new = self.allocator.allocate::<Node<T>>();

        let mut head = self.head;
        let mut first = unsafe { head.as_ref() }.next;

        unsafe {
            std::ptr::write(
                new.as_ptr(),
                Node {
                    next: first,
                    previous: head,
                    value: MaybeUninit::new(value),
                },
            );
            head.as_mut().next = new;
            first.as_mut().previous = new;
        }

        self.length = self.length.checked_add(1).expect("list length overflow");
    }

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
    /// `node` must be a node and belong this list and not be the sentinel node.
    unsafe fn detach_node(&mut self, node: NonNull<Node<T>>) -> T {
        debug_assert!(node != self.head, "attempted to remove sentinel node");

        let node = node.as_ptr();
        let prev = unsafe { (*node).previous };
        let next = unsafe { (*node).next };

        unsafe {
            (*prev.as_ptr()).next = next;
            (*next.as_ptr()).previous = prev;
        }

        self.length = self
            .length
            .checked_sub(1)
            .expect("list length went below 0");

        let value = unsafe { (*node).value.assume_init_read() };
        unsafe { self.allocator.deallocate_raw(node as _) };

        value
    }
}

impl<T, A: Allocator> Drop for List<T, A> {
    fn drop(&mut self) {
        let mut current = unsafe { self.head.as_ref() }.next;
        while current != self.head {
            let next = unsafe { current.as_ref() }.next;
            unsafe {
                std::ptr::drop_in_place((*current.as_ptr()).value.as_mut_ptr());
                self.allocator.deallocate_raw(current.as_ptr() as _);
            }
            current = next;
        }
        unsafe { self.allocator.deallocate_raw(self.head.as_ptr() as _) };
    }
}
