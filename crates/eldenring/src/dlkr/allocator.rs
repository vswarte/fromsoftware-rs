use crate::dlkr::{DLPlainLightMutex, DLPlainReadWriteLock, PlainAdaptiveMutexImpl};
use std::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    marker::PhantomData,
    mem::transmute,
    ptr::NonNull,
};

use pelite::pe64::Pe;
use shared::{OwnedPtr, Program};
use vtable_rs::VPtr;

use crate::rva;

#[vtable_rs::vtable]
pub trait DLAllocatorVmt {
    fn destructor(&mut self, param_2: bool);

    /// Getter for the allocator ID.
    fn allocator_id(&self) -> u32;

    fn unk10(&self);

    fn heap_flags(&self) -> &u64;

    fn heap_capacity(&self) -> usize;

    fn heap_size(&self) -> usize;

    fn backing_heap_capacity(&self) -> usize;

    fn heap_allocation_count(&self) -> usize;

    /// Retrieves allocation size for a specific allocation.
    fn allocation_size(&self, allocation: *const u8) -> usize;

    fn allocate(&mut self, size: usize) -> *const u8;

    fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned(
        &mut self,
        allocation: *const u8,
        size: usize,
        alignment: usize,
    ) -> *const u8;

    fn deallocate(&mut self, allocation: *const u8);

    fn allocate_second(&mut self, size: usize) -> *const u8;

    fn allocate_aligned_second(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate_second(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned_second(
        &mut self,
        allocation: *const u8,
        size: usize,
        alignment: usize,
    ) -> *const u8;

    fn deallocate_second(&mut self, allocation: *const u8);

    fn unka0(&self) -> bool;

    fn allocation_belongs_to_first_allocator(&mut self, allocation: *const u8) -> bool;

    fn allocation_belongs_to_second_allocator(&mut self, allocation: *const u8) -> bool;

    fn lock(&mut self);

    fn unlock(&mut self);

    fn get_memory_block_for_allocation(&mut self, allocation: *const u8) -> *const u8;
}

pub struct DLAllocatorBase {
    pub vftable: VPtr<dyn DLAllocatorVmt, Self>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DLAllocatorRef(NonNull<DLAllocatorBase>);

impl DLAllocatorRef {
    /// Returns the global instance of DLAllocator that uses the standard MSVC malloc()/free()
    /// implementation for heap management
    pub fn runtime_heap_allocator() -> Self {
        unsafe {
            transmute::<u64, Self>(
                Program::current()
                    .rva_to_va(rva::get().runtime_heap_allocator)
                    .unwrap(),
            )
        }
    }
}

unsafe impl GlobalAlloc for DLAllocatorRef {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = self.0.as_ptr();
        unsafe { ((*allocator).vftable.allocate)(&mut *allocator, layout.size()) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let allocator = self.0.as_ptr();
        unsafe {
            ((*allocator).vftable.deallocate)(&mut *allocator, ptr);
        }
    }
}

impl From<NonNull<DLAllocatorBase>> for DLAllocatorRef {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

impl DLAllocatorVmt for DLAllocatorBase {
    extern "C" fn destructor(&mut self, _param_2: bool) {
        todo!()
    }

    extern "C" fn allocator_id(&self) -> u32 {
        todo!()
    }

    extern "C" fn unk10(&self) {
        todo!()
    }

    extern "C" fn heap_flags(&self) -> &u64 {
        todo!()
    }

    extern "C" fn heap_capacity(&self) -> usize {
        todo!()
    }

    extern "C" fn heap_size(&self) -> usize {
        todo!()
    }

    extern "C" fn backing_heap_capacity(&self) -> usize {
        todo!()
    }

    extern "C" fn heap_allocation_count(&self) -> usize {
        todo!()
    }

    extern "C" fn allocation_size(&self, _allocation: *const u8) -> usize {
        todo!()
    }

    extern "C" fn allocate(&mut self, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8 {
        (self.vftable.allocate_aligned)(self, size, alignment)
    }

    extern "C" fn reallocate(&mut self, _allocation: *const u8, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_aligned(
        &mut self,
        _allocation: *const u8,
        _size: usize,
        _alignment: usize,
    ) -> *const u8 {
        todo!()
    }

    extern "C" fn deallocate(&mut self, _allocation: *const u8) {
        todo!()
    }

    extern "C" fn allocate_second(&mut self, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn allocate_aligned_second(&mut self, _size: usize, _alignment: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_second(&mut self, _allocation: *const u8, _size: usize) -> *const u8 {
        todo!()
    }

    extern "C" fn reallocate_aligned_second(
        &mut self,
        _allocation: *const u8,
        _size: usize,
        _alignment: usize,
    ) -> *const u8 {
        todo!()
    }

    extern "C" fn deallocate_second(&mut self, _allocation: *const u8) {
        todo!()
    }

    extern "C" fn unka0(&self) -> bool {
        todo!()
    }

    extern "C" fn allocation_belongs_to_first_allocator(&mut self, _allocation: *const u8) -> bool {
        todo!()
    }

    extern "C" fn allocation_belongs_to_second_allocator(
        &mut self,
        _allocation: *const u8,
    ) -> bool {
        todo!()
    }

    extern "C" fn lock(&mut self) {
        todo!()
    }

    extern "C" fn unlock(&mut self) {
        todo!()
    }

    extern "C" fn get_memory_block_for_allocation(&mut self, _allocation: *const u8) -> *const u8 {
        todo!()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NodeColor {
    Red = 0,
    Black = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaggedPtr<T, const TAG_SIZE: usize> {
    pub bits: usize,
    _marker: PhantomData<*mut T>,
}

impl<T, const TAG_SIZE: usize> TaggedPtr<T, TAG_SIZE> {
    const TAG_MASK: usize = (1 << TAG_SIZE) - 1;
    const PTR_MASK: usize = !Self::TAG_MASK;

    pub fn ptr(&self) -> Option<NonNull<T>> {
        NonNull::new((self.bits & Self::PTR_MASK) as *mut T)
    }

    pub fn tag(&self) -> usize {
        self.bits & Self::TAG_MASK
    }
}

pub type RBCPtr<T> = TaggedPtr<T, 1>;

impl<T> RBCPtr<T> {
    pub fn color(self) -> NodeColor {
        if !self.is_black() {
            NodeColor::Red
        } else {
            NodeColor::Black
        }
    }

    pub fn is_black(self) -> bool {
        self.tag() & 1 != 0
    }
}

#[repr(C)]
pub struct HeapAllocator<T> {
    pub base: DLAllocatorBase,
    /// Non-owning pointer to the heap this allocator wraps
    pub heap: NonNull<T>,
}

/// Identifier assigned by DLHeapManager to each registered heap
#[repr(transparent)]
pub struct DLHeapIdentifier(pub i32);

#[repr(C)]
pub struct DLDynamicHeap<T> {
    pub parent_allocator: Option<NonNull<DLAllocatorBase>>,
    pub heap_id: DLHeapIdentifier,
    pub buffer: *mut c_void,
    /// Self-referential allocator
    pub allocator: HeapAllocator<Self>,
    pub strategy: T,
}

/// Wraps heap T with a mutex (DLMultiThreadingPolicy).
/// Used for single-direction (front-only) allocation strategies
#[repr(C)]
pub struct DLDefaultHeapStrategy<T> {
    pub heap: T,
    pub sync: DLPlainLightMutex,
}

/// Same as DLDefaultHeapStrategy, but adds bidirectional (front + back)
/// allocation strategy
#[repr(C)]
pub struct DLBiHeapStrategy<T> {
    pub heap: T,
    pub sync: DLPlainLightMutex,
}

/// Intrusive free-list link.  Lives at `+0x10` inside a free HeapBlock
/// (i.e., at the start of the user payload area)
#[repr(C)]
pub struct HeapBlockLink {
    pub prev: *mut HeapBlock,
    pub next: *mut HeapBlock,
}

/// Physical memory block header.
///
/// Every allocation managed by DLRegularHeap begins with this header.
/// The user pointer returned by `Alloc()` points to `free_link`
/// (i.e., `+0x10` from the block start).
///
/// # Flag bits in `next_and_flags`
/// 0 `ALLOCATED` this block is live; 0 = free
/// 1 `PREV_ALLOC` the physically preceding block is allocated
///
/// Minimum free block size: 0x20 bytes (16-byte header + 16-byte free_link).
/// All blocks are 16-byte aligned
#[repr(C, align(16))]
pub struct HeapBlock {
    /// Contiguous block immediately behind this one in memory
    pub prev_physical: *mut HeapBlock,
    /// Pointer to next physical block with flag bits in bits 0–1
    pub next_and_flags: TaggedPtr<HeapBlock, 2>,
    /// When FREE: intrusive doubly-linked list node.
    /// When ALLOCATED: start of the user's data (the returned pointer)
    pub free_link: HeapBlockLink,
}

/// Boundary-tag allocator with 64 power-of-2 size classes.
///
/// `index = floor(log2(block_size)) − 1`  (block_size includes the 16-byte header)
/// `free_size_class_bitmap` bit `i` = 1 iff `free_table[i]` is non-empty
/// Every allocation carries a 16-byte header (`HeapBlock`).
/// Minimum allocation: 0x20 bytes total (0x10 header + 0x10 free_link minimum)
#[repr(C)]
pub struct DLRegularHeap {
    /// Bitmask: bit i set -> free_table[i] has >=1 free block
    pub free_size_class_bitmap: u64,
    pub start: *mut c_void,
    pub end: *mut c_void,
    /// Sum of all free bytes across every size class
    pub total_free_size: usize,
    /// Number of blocks currently held by callers
    pub allocated_block_count: usize,
    /// 64 size-class list heads (sentinels). `free_table[i].next` = first free block
    /// in class i, or points back to `&free_table[i]` when empty
    pub free_table: [HeapBlockLink; 64],
    /// Non-owning pointer to the currently largest single free block
    pub largest_free_block: *mut HeapBlockLink,
    /// Non-owning pointer to the physical end-of-heap sentinel block
    pub end_sentinel: *mut HeapBlockLink,
    pub max_free_block_size: usize,
}

impl DLRegularHeap {
    /// Returns true if the heap covers `ptr` (does not check allocated state).
    #[inline]
    pub fn contains(&self, ptr: *const u8) -> bool {
        let p = ptr as usize;
        let start = self.start as usize;
        let end = self.end as usize;
        p >= start && p < end
    }

    /// Returns the block size visible to the user for a live allocation at `ptr`.
    /// Returns None if the allocation flag (bit 0) is not set
    ///
    /// # Safety
    /// `ptr` must be a pointer previously returned by this heap's Alloc
    pub unsafe fn allocation_size(&self, ptr: NonNull<u8>) -> Option<usize> {
        let block = unsafe { ptr.as_ptr().sub(0x10) } as *const HeapBlock;
        let next_and_flags = unsafe { &block.as_ref()?.next_and_flags };
        if next_and_flags.tag() & 1 == 0 {
            return None;
        } // not allocated
        let next_ptr = next_and_flags.ptr()?.as_ptr();
        Some(next_ptr as usize - ptr.as_ptr() as usize)
    }

    pub fn total_free(&self) -> usize {
        self.total_free_size
    }
    pub fn allocated_blocks(&self) -> usize {
        self.allocated_block_count
    }
}

/// Red-black tree node for large free blocks.
///
/// Lives at `user_ptr + 0x00` (i.e., `HeapBlock + 0x10`) when the block is free.
/// The RBT is sorted by block size (ascending).
/// The sort key (block size) is stored at `user_ptr + 0x18`, AFTER
/// this struct, but is NOT a field here
///
/// # parent_color encoding
/// `bits[63:1]` = parent pointer
/// `bits[0]`    = color: 0 = RED, 1 = BLACK
#[repr(C)]
pub struct RobustFreeNode {
    pub left: *mut RobustFreeNode,
    pub right: *mut RobustFreeNode,
    pub parent_color: RBCPtr<RobustFreeNode>,
}

/// Sentinel for one of the 31 small-block size classes in DLRobustHeap.
///
/// The pool region at `pool_base` begins with an array of 31 of these,
/// occupying bytes `[0x000 .. 0x1F0)`. Pool objects start at `pool_base + 0x1F0`.
///
/// The list is circular: `sentinel.next_payload` points to the first free
/// item's payload, and that item's payload[-1] points back to the sentinel.
/// An empty class has both fields pointing to `self`.
#[repr(C)]
pub struct PoolSizeClassSentinel {
    /// Points to the LAST free item's payload, or `self` when empty
    pub prev_payload: *mut c_void,
    /// Points to the FIRST free item's payload, or `self` when empty
    pub next_payload: *mut c_void,
}

/// A dual-path heap:
///
/// - **Small path** (request < 0x1F9 bytes): 31 fixed-size pool classes at
///   16-byte granularity, served from `pool_base + 0x1F0` upward.
///   No coalescing, items are simply pushed/popped from their size-class list.
///
/// - **Large path** (request >= 0x1F9 bytes): a red-black tree of free blocks
///   sorted by size. Blocks are carved from the region growing downward from
///   `large_block_base`. Supports coalescing with adjacent free blocks
///
/// # Small block layout (returned pointer `P`):
/// ```text
/// P - 0x08:  next_and_flags  (usize; bit 0 = allocated)
/// P + 0x00:  user data       (or *mut next_free when free)
/// ```
///
/// # Large block layout (returned pointer `P`):
/// ```text
/// P - 0x10:  prev_physical   (*mut LargeFreeBlockOverlay)
/// P - 0x08:  next_and_flags  (usize; bit 0 = allocated, bit 1 = prev_alloc)
/// P + 0x00:  RobustFreeNode  (RBT node when free; user data when allocated)
/// P + 0x18:  free_block_size (usize; RBT key, only valid when free)
/// ```
#[repr(C)]
pub struct DLRobustHeap {
    /// Total byte span of the managed region; set once at init, never changes
    pub managed_span: usize,
    /// Remaining allocatable bytes
    pub total_free_capacity: usize,
    /// Monotonically increasing counter of allocation attempts. Decremented
    /// only on OOM
    pub allocation_metrics: usize,
    /// Base of the pool region.
    ///
    /// Memory map:
    /// ```text
    /// pool_base + 0x000 .. 0x1EF  -> [PoolSizeClassSentinel; 31]  (0x1F0 bytes)
    /// pool_base + 0x1F0 ..        -> pool objects grow upward ^
    /// ```
    pub pool_base: *mut c_void,
    /// Root of the RBT of large free blocks (sorted by size, ascending).
    /// NULL when no large free blocks are available
    pub rbt_root: *mut RobustFreeNode,
    /// High mark of committed pool memory. Advances when new pool slabs
    /// are carved from the managed region
    pub pool_current_top: *mut c_void,
    /// Low mark of the large-block region. Retreats as large blocks are
    /// carved from the top of the heap downward
    pub large_block_base: *mut c_void,
}

/// Header for one 256 KiB slab in DLSmallObjectHeapWrapper.
///
/// Precedes the slab's allocatable items. Items within a slab form a
/// singly-linked free stack: each free item's first word points to the
/// next free item (NULL = end of stack).
///
/// Slabs for the same size class are wired into a circular doubly-linked
/// list via `prev_slab`/`next_slab`. The `AllocatorBin` acts as the list
/// sentinel: `bin.sentinel_prev` points to `&tail_slab.prev_slab` and
/// `bin.sentinel_next` points to `&head_slab.prev_slab`. Subtracting
/// `0x18` (= `offset_of!(SmallSlabHeader, prev_slab)`) recovers the
/// header pointer from either sentinel field
#[repr(C)]
pub struct SmallSlabHeader {
    /// Head of free-item stack. NULL when slab is fully allocated
    pub free_head: *mut c_void,
    /// Items currently allocated from this slab
    pub alloc_count: usize,
    /// Physical base address of this slab's memory
    pub slab_base: *mut c_void,
    /// Previous slab in the AllocatorBin's circular list.
    /// `AllocatorBin::sentinel_prev` <-> `&tail_slab.prev_slab`
    pub prev_slab: *mut SmallSlabHeader,
    /// Next slab in the AllocatorBin's circular list
    pub next_slab: *mut SmallSlabHeader,
}

/// Per-size-class bin for DLSmallObjectHeapWrapper.
///
/// Acts as the circular-list sentinel for `SmallSlabHeader` nodes. An
/// empty bin has `sentinel_next == self as *mut AllocatorBin`.
///
/// # Empty check
/// `bin.sentinel_next == bin as *mut AllocatorBin`
///
/// # Getting the current slab header
/// When non-empty, `bin.sentinel_prev` points to the LAST slab's
/// `prev_slab` field. To get the LAST slab header:
/// `(bin.sentinel_prev as *mut SmallSlabHeader).sub(3)`
/// (subtract 3 * 8 = 0x18 = `offset_of!(SmallSlabHeader, prev_slab)`)
#[repr(C)]
pub struct AllocatorBin {
    /// Circular-list m_pPrev in the sentinel role.
    /// Non-empty: points to `last_slab.prev_slab`. Empty: points to `self`
    pub sentinel_prev: *mut c_void,
    /// Circular-list m_pNext in the sentinel role.
    /// Non-empty: points to `first_slab.prev_slab`. Empty: points to `self`
    pub sentinel_next: *mut c_void,
    /// Maximum item size this bin serves = `(bin_index + 1) * 16`
    pub max_item_size: usize,
    /// Items allocated per slab for this bin
    pub items_per_slab: usize,
}

/// A slab-allocator front-end that intercepts small requests.
///
/// Requests <= `BINS * 16` bytes are routed to the corresponding
/// `AllocatorBin` and served from 256 KiB slabs (`CHUNK_SIZE = 0x3FFF8`
/// aligned to `CHUNK_ALIGN = 0x40000`). Larger requests fall through to
/// `base_heap` directly.
///
/// # Const generics
/// `BINS` = number of size classes = `MAX_BIN_SIZE / 16`:
///
/// | Instantiation                             | BINS | Serves up to |
/// |-------------------------------------------|------|--------------|
/// | `DLSmallObjectHeapWrapper<T, 32>`         |  32  | 512 bytes    |
/// | `DLSmallObjectHeapWrapper<T, 16>`         |  16  | 256 bytes    |
///
/// # Bitmap
/// `alloc_region_bitmap` is an array of `u64` covering the managed span
/// at 256 KiB granularity. Bit `(offset >> 18) % 64` in
/// `bitmap[(offset >> 24)]` is set when that 256 KiB chunk has been
/// committed. Used to answer ownership queries
#[repr(C)]
pub struct DLSmallObjectHeapWrapper<T, const BINS: usize> {
    /// Underlying heap for slab provisioning and large (> BINS*16 byte) requests.
    pub base_heap: T,
    /// Bitmap array tracking committed 256 KiB chunks. Non-owning; points into
    /// a region carved from `base_heap`
    pub alloc_region_bitmap: *mut c_void,
    unk: usize,
    /// Aligned base used for bitmap offset calculations
    pub alloc_region_start_aligned: *mut c_void,
    /// Size-class bins. `bins[(request - 1) >> 4]` selects the bin for a request
    pub bins: [AllocatorBin; BINS],
}

/// A slab-of-slabs allocator used internally by both
/// `DLSegregatedRegularHeap` and `DLSegregatedBiHeapStrategy` to
/// provide pre-formatted chunk/block descriptor objects cheaply
///
/// Descriptors are carved from slabs allocated via `parent_allocator`.
/// Exhausted slabs are unlinked; their descriptors are returned to the
/// slab's free stack when freed
///
/// # Self-referential sentinel
/// `head_slab` is initialized to `&self.head_slab` (its own address).
/// When a slab's `next_slab` pointer equals `&self.head_slab`, that slab
/// is the tail (it's the "end-of-list" sentinel value, not a node)
#[repr(C)]
pub struct DLFixedAllocator<T> {
    /// Cached pointer to the slab currently being drained. NULL if all slabs
    /// are either empty or the cache was explicitly cleared
    pub current_slab: *mut DLFixedSlab<T>,
    /// Cached pointer into the last-accessed slab's RBT node
    /// (`&slab.rbt_node.parent_color`). NULL when stale
    pub cached_slab_pos: *mut c_void,
    /// Byte distance between consecutive items within a slab
    /// (= `align_up(size_of::<T>(), slab_alignment)`)
    pub item_stride: usize,
    /// Total bytes requested from `parent_allocator` per slab
    /// (= `items_per_slab * item_stride + size_of::<DLFixedSlab<T>>()`)
    pub slab_alloc_size: usize,
    /// Alignment passed to `parent_allocator.allocate_aligned` for each slab
    pub slab_alignment: usize,
    /// Number of items of type `T` carved from each newly allocated slab
    pub items_per_slab: usize,
    /// Total free items currently available across all live slabs
    pub total_free_items: usize,
    /// Number of slabs currently allocated from `parent_allocator`
    pub slab_count: usize,
    /// Non-owning pointer to the parent allocator used for slab provisioning
    pub parent_allocator: *mut DLAllocatorBase,
    /// Head of the doubly-linked slab list.
    /// Initialized to `&self.head_slab` (self-referential = empty).
    /// When a slab's `next_slab == &self.head_slab`, it is the tail sentinel.
    pub head_slab: *mut DLFixedSlab<T>,
    /// Tail of the doubly-linked slab list
    pub tail_slab: *mut DLFixedSlab<T>,
    /// Root of an RBT keyed on slab base address, used by `Push` to find
    /// the owning slab for a returned item in O(log n)
    pub slab_rbt_root: *mut RobustFreeNode,
}

impl<T> DLFixedAllocator<T> {
    /// True when all items across all slabs have been handed out.
    /// A `Pop` call will trigger `GrowSlab` to provision a new slab.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.total_free_items == 0
    }

    /// True when at least one slab has been allocated from `parent_allocator`.
    /// False means `head_slab` is the self-referential sentinel (no real slabs).
    #[inline]
    pub fn has_slabs(&self) -> bool {
        !self.head_slab.is_null()
            && self.head_slab
                != (&self.head_slab as *const *mut DLFixedSlab<T> as *mut DLFixedSlab<T>)
    }
}

/// One slab of pre-allocated items of type `T` managed by `DLFixedAllocator`.
///
/// Layout within the allocation of size `slab_alloc_size`:
/// ```text
/// +0x00 .. items_per_slab * item_stride  : item pool (free stack threaded through first word)
/// slab_alloc_size - 0x38:  DLFixedSlab header (this struct)
/// ```
/// `GrowSlab` places the header at `alloc_ptr + (slab_alloc_size - 0x38)`.
///
/// Items within the slab are singly linked through each item's first word.
/// `free_head` points to the first free item; when NULL the slab is exhausted.
/// `free_count` is stored immediately after the header (at `+0x38` relative to
/// `DLFixedSlab` base), accessed in decompiled code as `slab[1].prev_slab`.
#[repr(C)]
pub struct DLFixedSlab<T> {
    /// Previous slab in the allocator's doubly-linked list
    pub prev_slab: *mut DLFixedSlab<T>,
    /// Next slab, or `&allocator.head_slab` if this is the tail sentinel
    pub next_slab: *mut DLFixedSlab<T>,
    /// RBT node keyed on this slab's base address, used by `Push` for O(log n)
    /// lookup of the owning slab during deallocation
    pub rbt_node: RobustFreeNode,
    /// Head of the free-item stack for this slab. NULL when fully allocated.
    /// Each free item's first word points to the next free item.
    pub free_head: *mut T,
    /// Number of free items remaining in this slab
    pub free_count: usize,
}

/// Circular free-list sentinel for one size class in `DLSegregatedRegularHeap`.
///
/// Links point to `&SegChunkDesc::free_link` (at `+0x18` within the descriptor).
/// To recover the descriptor from a link pointer,
/// subtract `offset_of!(SegChunkDesc, free_link)` (= `0x18`).
///
/// An empty class has both fields pointing to `self` (i.e., `&free_table[i]`).
///
/// # Size-class walk
/// Allocation scans from `head` following `SegChunkFreeLink::prev_or_hash_next`.
/// Insertion prepends at `head`
#[repr(C)]
pub struct SegFreeListHead {
    /// Points to `&first_free_desc.free_link`, or `self` when empty.
    /// Corresponds to `SegChunkFreeLink::prev_or_hash_next` position.
    pub head: *mut SegChunkFreeLink,
    /// Points to `&last_free_desc.free_link`, or `self` when empty.
    /// Corresponds to `SegChunkFreeLink::next` position.
    pub tail: *mut SegChunkFreeLink,
}

/// Intrusive link embedded at `+0x18` inside `SegChunkDesc`.
///
/// # Dual use depending on descriptor state
///
/// ## Free state (`next != NULL`)
/// Part of a circular doubly-linked list per size class.
/// - `prev_or_hash_next` -> previous free entry's `&free_link`
///   (or `&free_table[class]` if this is the first entry)
/// - `next` -> next free entry's `&free_link`
///   (or `&free_table[class]` if this is the last entry)
///
/// The sentinel `SegFreeListHead` occupies the same 16-byte layout,
/// so the list is seamlessly circular through the sentinel.
///
/// ## Allocated state (`next == NULL`)
/// - `prev_or_hash_next` -> next `SegChunkDesc*` in the hash bucket chain
///   (or NULL if this is the last entry in the bucket)
/// - `next` -> NULL (marks this descriptor as allocated)
#[repr(C)]
pub struct SegChunkFreeLink {
    /// Free: previous free entry's `&free_link` (or sentinel).
    /// Allocated: next descriptor in hash bucket chain (or NULL).
    pub prev_or_hash_next: *mut c_void,
    /// Free: next free entry's `&free_link` (or sentinel). NULL = allocated.
    pub next: *mut c_void,
}

/// Descriptor for one contiguous extent managed by `DLSegregatedRegularHeap`.
///
/// Extents are kept in a circular doubly-linked physical-address list
/// anchored at `DLSegregatedRegularHeap::sentinel`. Free extents are
/// additionally threaded into per-class circular lists via `free_link`.
///
/// # States
/// - **Free**: `free_link.next != NULL` (points to sentinel or next free entry)
/// - **Allocated**: `free_link.next == NULL`; `free_link.prev_or_hash_next`
///   is used as the hash bucket chain link
///
/// # Free-list geometry
/// `SegFreeListHead` and `SegChunkFreeLink` share the same 2-pointer layout.
/// Free-list pointers target `&desc.free_link` (at `+0x18`), not the
/// descriptor base. To recover the descriptor:
/// ```text
/// desc = (free_link_ptr as usize - 0x18) as *mut SegChunkDesc
/// ```
#[repr(C)]
pub struct SegChunkDesc {
    /// Next extent in physical-address order (circular through sentinel)
    pub phys_next: *mut SegChunkDesc,
    /// Previous extent in physical-address order (circular through sentinel)
    pub phys_prev: *mut SegChunkDesc,
    /// Start address of this extent. For allocated extents, this is the
    /// pointer returned to the caller.
    pub chunk_start: *mut c_void,
    /// Size-class free-list link (when free) or hash-chain link (when allocated).
    /// See `SegChunkFreeLink` for dual-use semantics.
    pub free_link: SegChunkFreeLink,
}

impl SegChunkDesc {
    pub fn is_free(&self) -> bool {
        !self.free_link.next.is_null()
    }

    /// # Safety
    /// `phys_next` must point to the physically next extent or the heap sentinel.
    pub unsafe fn chunk_end(&self) -> *mut c_void {
        unsafe { (*self.phys_next).chunk_start }
    }

    /// # Safety
    /// Same requirements as `chunk_end()`.
    pub unsafe fn chunk_size(&self) -> usize {
        unsafe { self.chunk_end() as usize - self.chunk_start as usize }
    }
}

/// Segregated free-list heap with 64 size classes and a hash table for
/// O(1) allocated-block lookup.
///
/// # Size classes
/// - Small (`< 0x201` bytes): `class = (align16(size) - 1) >> 4` (32 classes, 16-byte granularity)
/// - Large (`>= 0x201` bytes): `class = floor(log2(align16(size))) + 0x17` (32 classes)
///
/// Both ranges share the same 64-entry `free_table` and 64-bit bitmap.
///
/// # Hash table
/// `hash_table` is a flat array of `0x44F` (1103) bucket heads, allocated
/// as a single `0x2278`-byte block from the parent allocator.
/// Key: `chunk_start as usize % 0x44F`. Chains use `SegChunkFreeLink::prev_or_hash_next`.
///
/// # Physical list
/// All extents (free and allocated) are in a circular doubly-linked list
/// ordered by address, anchored at `sentinel`. `sentinel.chunk_start = range_end`.
///
/// # Coalescing
/// On free, if the physically adjacent extent(s) are also free, they are
/// merged. Redundant descriptors are returned to `chunk_recycler`.
///
/// # Allocation failure path
/// If `DLFixedAllocator` fails to provide a remainder descriptor during a split, the
/// original descriptor is re-inserted into the free list for its full
/// (un-split) size class and `NULL` is returned to the caller
#[repr(C)]
pub struct DLSegregatedRegularHeap {
    /// Total free bytes currently available (decremented on alloc, incremented on free)
    pub total_free_capacity: usize,
    /// Total aligned span of the managed range; set once at init, never modified
    pub managed_span: usize,
    /// Bitmask: bit `i` is set iff `free_table[i]` has at least one entry
    pub size_class_bitmap: u64,
    /// Number of currently live (unreturned) allocations
    pub allocation_count: usize,
    /// Physical-list sentinel. `sentinel.chunk_start = range_end`.
    /// All extents form a circular list through this sentinel.
    /// `sentinel.free_link` is zeroed (not part of any free list).
    pub sentinel: SegChunkDesc,
    /// Hash table for allocated-block lookup.
    /// `0x44F` buckets, allocated as one `0x2278`-byte block.
    /// Key: `chunk_start as usize % 0x44F`.
    /// Each bucket is a singly-linked list via `SegChunkFreeLink::prev_or_hash_next`
    pub hash_table: *mut [*mut SegChunkDesc; 0x44F],
    /// Aligned start of the managed address range
    pub range_start: *mut c_void,
    /// Aligned end of the managed address range (= `sentinel.chunk_start`)
    pub range_end: *mut c_void,
    /// Per-class circular free-list sentinels.
    /// Each entry's links point to `&desc.free_link`, not descriptor bases.
    pub free_table: [SegFreeListHead; 64],
    /// Fixed-size allocator providing `SegChunkDesc` descriptors.
    /// Initialized with `(block_size=0x28, chunk_size=0x1000, align=8)`.
    pub chunk_recycler: DLFixedAllocator<SegChunkDesc>,
}

/// Descriptor for one contiguous free address range managed by
/// `DLSegregatedBiHeapStrategy`.
///
/// Blocks are kept in both a physical-address-ordered doubly-linked list and a
/// free-address-ordered doubly-linked list. The strategy does not own the
/// address range itself; it just tracks it.
///
/// # Implicit "end" address
/// A block's usable extent is `[region_start .. phys_next.region_start)`.
/// There is no explicit `region_end` field; the end is the next physical block's start.
///
/// # States
/// - **Free**: has valid `free_prev`/`free_next` links (non-null).
/// - **Allocated (spent)**: pushed onto `DLSegregatedBiHeapStrategy::spent_block_stack`,
///   `free_prev = NULL`, linked via `free_next`.
/// - **Allocated (extended)**: on `extended_alloc_list`, `free_prev = NULL`,
///   linked via `free_next`.
#[repr(C)]
pub struct SegBiBlock {
    /// Start address of this block's region
    pub region_start: *mut c_void,
    /// Previous block in physical-address order
    pub phys_prev: *mut SegBiBlock,
    /// Next block in physical-address order (its `region_start` = this block's end)
    pub phys_next: *mut SegBiBlock,
    /// Previous block in the free-address doubly-linked list.
    /// NULL when this block is allocated (on spent_block_stack or extended_alloc_list).
    /// Points to `&sentinel` when this is the first free block.
    pub free_prev: *mut SegBiBlock,
    /// Next block in the free-address doubly-linked list.
    /// When allocated: reused as the "next" link in the LIFO stack
    /// (spent_block_stack or extended_alloc_list).
    /// Points to `&sentinel` when this is the last free block.
    pub free_next: *mut SegBiBlock,
}

/// An address-range splitting heap strategy (`DLSegregatedBiHeap`).
///
/// Manages a collection of physically contiguous free ranges (`SegBiBlock`s).
/// Allocation splits a range into up to 3 sub-ranges:
///   1. Pre-alignment padding (inserted into free list or dropped)
///   2. The allocation itself (removed from free list, pushed to spent_block_stack)
///   3. Remainder (inserted into free list at new address)
///
/// Supports arbitrary alignment via address arithmetic.
///
/// # Free-list sentinel
/// `sentinel` is a dummy `SegBiBlock` embedded in the struct.
/// `sentinel.region_start` is set to `range_end` at init.
/// An empty free list has `sentinel.free_next == &self.sentinel`.
///
/// # Allocation tracking
/// Spent (fully consumed range) blocks go to `spent_block_stack` (LIFO via `free_next`).
/// Blocks that could not be tracked normally go to `extended_alloc_list`.
/// Both lists have `free_prev == NULL` on each node.
///
/// # Block descriptor recycling
/// `block_recycler` (`DLFixedAllocator`) provides `SegBiBlock` descriptors.
/// Before allocating a new descriptor, the allocator pops from `spent_block_stack`.
#[repr(C)]
pub struct DLSegregatedBiHeapStrategy {
    /// Slab allocator providing `SegBiBlock` descriptor objects.
    pub block_recycler: DLFixedAllocator<SegBiBlock>,
    _pad60: [u8; 7],
    pub is_initialized: bool,
    /// Embedded sentinel `SegBiBlock`. The free list is empty when
    /// `sentinel.free_next == &self.sentinel`.
    /// `sentinel.region_start` is set to `range_end` at init.
    pub sentinel: SegBiBlock,
    /// LIFO stack of spent `SegBiBlock` descriptors (ranges fully handed out).
    /// Linked through `SegBiBlock::free_next`. `free_prev` is NULL. NULL = empty.
    pub spent_block_stack: *mut SegBiBlock,
    /// List of allocated blocks that overflowed the normal tracking path.
    /// Linked through `SegBiBlock::free_next`. `free_prev` is NULL. NULL = empty.
    pub extended_alloc_list: *mut SegBiBlock,
    /// Aligned start of the managed address range
    pub range_start: *mut c_void,
    /// Aligned end of the managed address range
    pub range_end: *mut c_void,
    /// Number of currently live (unreturned) allocations
    pub live_allocation_count: usize,
    /// Cumulative free bytes returned to the heap over its lifetime
    pub total_freed_bytes: usize,
    /// Mutex injected by `DLMultiThreadingPolicy`
    pub sync: DLPlainLightMutex,
}

// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLBiHeapStrategy<DLKR::DLSmallObjectHeapWrapper<DLKR::DLRobustHeap,0,262144,512,16>,DLKR::DLMultiThreadingPolicy>>>
pub type MainHeapAllocator = HeapAllocator<
    DLDynamicHeap<DLBiHeapStrategy<DLSmallObjectHeapWrapper<DLRobustHeap, { 512 / 16 }>>>,
>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLBiHeapStrategy<DLKR::DLSmallObjectHeapWrapper<DLKR::DLRobustHeap,1,16384,512,16>,DLKR::DLMultiThreadingPolicy>>>
pub type GFXHeapAllocator = HeapAllocator<
    DLDynamicHeap<DLBiHeapStrategy<DLSmallObjectHeapWrapper<DLRobustHeap, { 512 / 16 }>>>,
>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>>>
pub type GFXTempHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>>>
pub type InGameHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>>>
pub type TempHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>>>
pub type CoreResHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLBiHeapStrategy<DLKR::DLRobustHeap,DLKR::DLMultiThreadingPolicy>>>
pub type MoWwiseHeapAllocator = HeapAllocator<DLDynamicHeap<DLBiHeapStrategy<DLRobustHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLBiHeapStrategy<DLKR::DLRobustHeap,DLKR::DLMultiThreadingPolicy>>>
pub type MoWwiseMoOnlyHeapAllocator = HeapAllocator<DLDynamicHeap<DLBiHeapStrategy<DLRobustHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRobustHeap,DLKR::DLMultiThreadingPolicy>_>_>
// typo in FS code
pub type MoWwiseIsorationHeapAllocator =
    HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRobustHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>_>_>
pub type LuaHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRobustHeap,DLKR::DLMultiThreadingPolicy>_>_>
pub type HavokHeapAllocator = HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRobustHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLBiHeapStrategy<DLKR::DLSmallObjectHeapWrapper<DLKR::DLRobustHeap,1,16384,512,16>,DLKR::DLMultiThreadingPolicy>_>_>
pub type MenuHeapAllocator = HeapAllocator<
    DLDynamicHeap<DLBiHeapStrategy<DLSmallObjectHeapWrapper<DLRobustHeap, { 512 / 16 }>>>,
>;

#[repr(C)]
pub struct CSNetworkAllocator {
    pub base: DLAllocatorBase,
    pub undelying: NonNull<HeapAllocator<DLDynamicHeap<DLBiHeapStrategy<DLRobustHeap>>>>,
}

// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLSmallObjectHeapWrapper<DLKR::DLRobustHeap,0,16384,256,16>,DLKR::DLMultiThreadingPolicy>>>
pub type DebugHeapAllocator = HeapAllocator<
    DLDynamicHeap<DLDefaultHeapStrategy<DLSmallObjectHeapWrapper<DLRobustHeap, { 256 / 16 }>>>,
>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLDefaultHeapStrategy<DLKR::DLRegularHeap,DLKR::DLMultiThreadingPolicy>_>_>
pub type GFXSystemSharedHeapAllocator =
    HeapAllocator<DLDynamicHeap<DLDefaultHeapStrategy<DLRegularHeap>>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLSegregatedBiHeapStrategy<DLKR::DLMultiThreadingPolicy>_>_>
pub type GFXGraphicsPrivateAHeapAllocator =
    HeapAllocator<DLDynamicHeap<DLSegregatedBiHeapStrategy>>;
// HeapAllocator<DLKR::DLDynamicHeap<DLKR::DLSegregatedRegularHeap<DLKR::DLMultiThreadingPolicy>_>_>
pub type GFXGraphicsPrivateBHeapAllocator = HeapAllocator<DLDynamicHeap<DLSegregatedRegularHeap>>;

#[repr(C)]
pub struct CSGraphicsPrivateAllocator {
    pub base: DLAllocatorBase,
    pub private_a: NonNull<GFXGraphicsPrivateAHeapAllocator>,
    pub private_b: NonNull<GFXGraphicsPrivateBHeapAllocator>,
    pub sync: PlainAdaptiveMutexImpl,
}

#[repr(C)]
/// Heap representing all other heaps.
pub struct DLSystemHeapImpl {
    pub base: DLRegularHeap,
    pub sync: DLPlainLightMutex,
    pub allocator: HeapAllocator<Self>,
    pub heap_registry: OwnedPtr<HeapRegistry>,
    pub registry_lock: DLPlainReadWriteLock,
    pub last_heap_id: DLHeapIdentifier,
}

#[repr(C)]
// TODO: replace with Vector<HeapAllocatorEntry, DLSystemHeapImpl> when #263 is merged
pub struct HeapRegistry {
    pub allocator: NonNull<DLSystemHeapImpl>,
    pub start: Option<NonNull<HeapAllocatorEntry>>,
    pub end: Option<NonNull<HeapAllocatorEntry>>,
    pub cap: Option<NonNull<HeapAllocatorEntry>>,
}

#[repr(C)]
pub struct HeapAllocatorEntry {
    pub allocator: NonNull<DLAllocatorBase>,
    pub heap_start: *mut c_void,
    pub heap_end: *mut c_void,
}
