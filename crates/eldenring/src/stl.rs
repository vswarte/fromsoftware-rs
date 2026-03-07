use std::ptr::NonNull;

use shared::OwnedPtr;

use crate::dlkr::DLAllocatorBase;

#[derive(Clone)]
#[repr(transparent)]
/// Special type to use in std types.
pub struct DLAllocatorForStl(NonNull<DLAllocatorBase>);

impl From<NonNull<DLAllocatorBase>> for DLAllocatorForStl {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

impl fromsoftware_shared_stl::Allocator for DLAllocatorForStl {
    fn allocate_raw(&mut self, size: usize, allign: usize) -> NonNull<std::ffi::c_void> {
        let allocator = self.0.as_ptr();
        let allocation =
            unsafe { ((*allocator).vftable.allocate_aligned)(&mut *allocator, size, allign) };
        if allocation.is_null() {
            panic!("DLAllocator returned null pointer")
        }
        unsafe { NonNull::new_unchecked(allocation as _) }
    }

    fn deallocate_raw(&mut self, ptr: *mut std::ffi::c_void) {
        let allocator = self.0.as_ptr();
        unsafe {
            ((*allocator).vftable.deallocate)(&mut *allocator, ptr as _);
        }
    }
}

pub type DLList<T> = fromsoftware_shared_stl::List<T, DLAllocatorForStl>;

pub type DLVector<T> = fromsoftware_shared_stl::Vector<T, DLAllocatorForStl>;

pub type DLMap<K, V> = fromsoftware_shared_stl::Map<K, V, DLAllocatorForStl>;
pub type DLMultiMap<K, V> = fromsoftware_shared_stl::MultiMap<K, V, DLAllocatorForStl>;

pub type DLSet<V> = fromsoftware_shared_stl::Set<V, DLAllocatorForStl>;
pub type DLMultiSet<V> = fromsoftware_shared_stl::MultiSet<V, DLAllocatorForStl>;

/// Special type for yet unspecified Red/Black tree where only allocator is known.
pub type UnkDLTree<V> =
    fromsoftware_shared_stl::RbTree<V, fromsoftware_shared_stl::Less, DLAllocatorForStl>;

#[repr(C)]
pub struct BasicVector<T>
where
    T: Sized,
{
    pub begin: Option<NonNull<T>>,
    pub end: Option<NonNull<T>>,
    pub capacity: Option<NonNull<T>>,
}

impl<T> BasicVector<T>
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
pub struct ChainingMap<K, V> {
    base: DLMap<K, NonNull<ChainingMapBucketEntry<V>>>,
    buckets: OwnedPtr<ArrayWithHeader<ChainingMapBucketEntry<V>>>,
}

impl<K, V> ChainingMap<K, V> {
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
            let key = &pair.first;
            let bucket = unsafe { pair.second.as_ref() };
            bucket.iter().map(move |value| (key, value))
        })
    }

    /// Iterates over all key-value pairs mutably, including all values in collision chains.
    /// Returns an iterator that yields `(&K, &mut V)` for each entry.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.base.iter_mut().flat_map(|pair| {
            let key = &pair.first;
            let bucket = unsafe { pair.second.as_mut() };
            bucket.iter_mut().map(move |value| (key, value))
        })
    }

    /// Iterates over keys and their collision chain heads.
    /// Use this if you need to iterate collision chains separately.
    pub fn iter_chains(&self) -> impl Iterator<Item = (&K, &ChainingMapBucketEntry<V>)> {
        self.base
            .iter()
            .map(|pair| (&pair.first, unsafe { pair.second.as_ref() }))
    }

    /// Iterates over keys and their collision chain heads mutably.
    /// Use this if you need to iterate collision chains separately.
    pub fn iter_chains_mut(
        &mut self,
    ) -> impl Iterator<Item = (&K, &mut ChainingMapBucketEntry<V>)> {
        self.base
            .iter_mut()
            .map(|pair| (&pair.first, unsafe { pair.second.as_mut() }))
    }

    pub fn buckets(&self) -> &[ChainingMapBucketEntry<V>] {
        unsafe { self.buckets.as_slice() }
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
