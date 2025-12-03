use std::ptr::NonNull;

use bitfield::bitfield;

use crate::fd4::FD4BasicHashString;
use shared::{Subclass, Superclass};

/// Represents a managed resource.
/// The data it represents is immediately handed over to
/// other systems and the ResCap serves as a token for unloading things.
/// One such example is gparams where the file associated with a FileCap is
/// parsed, ResCaps (multiple) are created from the FileCap, and the ResCaps
/// individually post the data they represent to associated sub-systems.
/// For GParamResCaps that means posting the such data to the gparam blending
/// system as well as a bunch of other GX structures.
///
/// Source of name: RTTI
#[repr(C)]
#[derive(Superclass)]
pub struct FD4ResCap {
    vftable: usize,
    /// Name of the resource contained in the ResCap
    pub name: FD4BasicHashString,
    /// The repository this resource is hosted in.
    pub owning_repository: Option<NonNull<FD4ResCapHolder<FD4ResCap>>>,
    /// Next item in the linked list
    pub next_item: Option<NonNull<FD4ResCap>>,
    /// Amount of references to this resource.
    pub reference_count: u32,
    unk5c: u32,
    unk60: bool,
    unk61: [u8; 7],
    unk68: usize,
    unk70: u8,
    unk71: [u8; 7],
}

/// Manages a collection of ResCaps by wrapping a FD4ResCapHolder and defines some logic specific
/// to T.
///
/// Source of name: RTTI
#[repr(C)]
#[derive(Superclass, Subclass)]
pub struct FD4ResRep<T>
where
    T: Subclass<FD4ResCap>,
{
    /// Repositories themselves inherit from ResCaps.
    pub res_cap: FD4ResCap,

    /// Holds a set of ResCaps wrapping T.
    pub res_cap_holder: FD4ResCapHolder<T>,
}

/// Represents a collection of ResCaps/FileCaps.
/// The game relies heavily on hashmaps for asset management.
/// The resources name gets turned in a u32 using some FNV variant. That hash
/// is then modulo'd by the repository's capacity to find the appropriate bucket.
/// In the case of collision on lookups it will start cycling through the
/// linked list for the matched slot and compare the full resource name hashes.
///
/// This fnv hashing itself is actually facilitated by FD4BasicHashString.
/// In the case of a collision on insertion it will make the entry you are
/// seeking to insert the new head.
///
/// Bucket # = fnv(resource name) % bucket count
///
/// +----------------------------------------------------------....
/// |               FD4ResCapHolder<T>'s map
/// +----------------------------------------------+-----------....
/// |  Bucket 0     |  Bucket 1     |  Bucket 2    |  Bucket 3
/// +---------------+---------------+--------------+-----------....
/// |  FD4ResCap    |  FD4ResCap    |              |  FD4ResCap
/// |  FD4ResCap    |               |              |  FD4ResCap
/// |  FD4ResCap    |               |              |
/// |               |               |              |
/// |               |               |              |
/// +---------------+---------------+--------------+-----------....
///
#[repr(C)]
pub struct FD4ResCapHolder<T>
where
    T: Subclass<FD4ResCap>,
{
    vftable: usize,
    allocator: usize,
    pub owning_repository: Option<NonNull<FD4ResCapHolder<FD4ResCap>>>,
    unk18: u32,
    pub bucket_count: u32,
    buckets: NonNull<Option<NonNull<T>>>,
}

impl<T> FD4ResCapHolder<T>
where
    T: Subclass<FD4ResCap>,
{
    /// Immutable iterator over entries.
    pub fn entries<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        // For immutable iteration we can store the current chain pointer (if any)
        // and an index into the bucket array.
        struct Iter<'a, T: Subclass<FD4ResCap>> {
            buckets_ptr: *const Option<NonNull<T>>,
            bucket_count: usize,
            current_bucket: usize,
            current_ptr: Option<NonNull<T>>,
            _marker: std::marker::PhantomData<&'a T>,
        }

        impl<'a, T: Subclass<FD4ResCap>> Iterator for Iter<'a, T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    // If there is no current pointer, try to advance to the next bucket.
                    while self.current_ptr.is_none() && self.current_bucket < self.bucket_count {
                        let bucket = *self.buckets_ptr.add(self.current_bucket);
                        self.current_bucket += 1;
                        if bucket.is_some() {
                            self.current_ptr = bucket;
                            break;
                        }
                    }
                    // If we have an element, yield it and update current_ptr from its chain.
                    if let Some(ptr) = self.current_ptr {
                        let item = ptr.as_ref();
                        // Copy the next pointer (avoiding borrowing the field).
                        // It's safe to cast here because we know everything in
                        // the container is (a subclass of) T.
                        let next = item.superclass().next_item.map(|p| p.cast());
                        self.current_ptr = next;
                        Some(item)
                    } else {
                        None
                    }
                }
            }
        }

        let buckets_ptr = self.buckets.as_ptr() as *const Option<NonNull<T>>;
        let bucket_count = self.bucket_count as usize;
        Iter {
            buckets_ptr,
            bucket_count,
            current_bucket: 0,
            current_ptr: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn entries_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'a {
        struct IterMut<'a, T: Subclass<FD4ResCap>> {
            buckets_ptr: *const Option<NonNull<T>>,
            bucket_count: usize,
            current_bucket: usize,
            current_ptr: Option<NonNull<T>>,
            _marker: std::marker::PhantomData<&'a mut T>,
        }

        impl<'a, T: Subclass<FD4ResCap>> Iterator for IterMut<'a, T> {
            type Item = &'a mut T;
            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    // If there's no current chain element, advance to the next bucket.
                    while self.current_ptr.is_none() && self.current_bucket < self.bucket_count {
                        let bucket = *self.buckets_ptr.add(self.current_bucket);
                        self.current_bucket += 1;
                        if bucket.is_some() {
                            self.current_ptr = bucket;
                            break;
                        }
                    }
                    // If we have an element, yield it and update from its chain.
                    if let Some(mut ptr) = self.current_ptr {
                        // Obtain a mutable reference from the pointer.
                        // This is safe because our iterator holds unique access.
                        let item = ptr.as_mut();
                        // Copy out the next pointer.
                        let next = item.superclass_mut().next_item.map(|p| p.cast());
                        self.current_ptr = next;
                        Some(item)
                    } else {
                        None
                    }
                }
            }
        }

        // Note: Although self.buckets is stored as NonNull<Option<NonNull<T>>>,
        // we only need its pointer for bucket iteration.
        let buckets_ptr = self.buckets.as_ptr() as *const Option<NonNull<T>>;
        let bucket_count = self.bucket_count as usize;
        IterMut {
            buckets_ptr,
            bucket_count,
            current_bucket: 0,
            current_ptr: None,
            _marker: std::marker::PhantomData,
        }
    }
}

/// Represents file load state for this FD4FileCap.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FD4FileCapState {
    Initial = 0x0,
    Queued = 0x1,
    Processing = 0x2,
    Unknown = 0x3,
    Ready = 0x4,
}

bitfield! {
    pub struct FD4FileCapUnk89Properties(u8);
    impl Debug;

    pub file_load_queue_index, set_file_load_queue_index: 4, 2;
}

bitfield! {
    pub struct FD4FileCapUnk8AProperties(u16);
    impl Debug;

    pub use_secondary_repository, set_use_secondary_repository: 1;

    u16;
    pub mutex_index, set_mutex_index: 15, 3;
}

/// Represents a file resource be it on-disk or virtual. Responsible for parsing the files bytes
/// and spawning ResCaps for the parsed resources.
///
/// Source of name: RTTI
#[repr(C)]
#[derive(Superclass, Subclass)]
pub struct FD4FileCap {
    pub res_cap: FD4ResCap,
    load_process: usize,
    load_task: usize,
    pub load_state: FD4FileCapState,
    unk89: FD4FileCapUnk89Properties,
    unk8a: FD4FileCapUnk8AProperties,
    unk8c: u32,
}
