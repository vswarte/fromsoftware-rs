use std::{
    alloc::{Layout, alloc, dealloc},
    ffi::c_void,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

use fromsoftware_shared_stl::Allocator;

#[derive(Clone)]
pub struct StdAlloc {
    live: &'static AtomicUsize,
}

impl StdAlloc {
    pub fn new(counter: &'static AtomicUsize) -> Self {
        StdAlloc { live: counter }
    }
    pub fn live_count(&self) -> usize {
        self.live.load(Ordering::Relaxed)
    }
}

/// Store the layout before the allocation so it can be recovered on
/// dealloc without a separate side table
///
/// ```text
///  [ usize: original size | usize: original align | ... user data ... ]
///  ^                                                ^
///  allocation start                          returned pointer
/// ```
const HEADER: usize = std::mem::size_of::<[usize; 2]>();

impl Allocator for StdAlloc {
    unsafe fn allocate_raw(&mut self, size: usize, align: usize) -> NonNull<c_void> {
        // Allocate with extra room for the header
        let full_align = align.max(std::mem::align_of::<usize>());
        let full_size = HEADER + size;
        let layout = Layout::from_size_align(full_size, full_align).unwrap();
        let raw = unsafe { alloc(layout) };
        assert!(!raw.is_null(), "allocation failed");
        // Write size and align into header
        unsafe {
            (raw as *mut usize).write(size);
            (raw as *mut usize).add(1).write(align);
        }
        self.live.fetch_add(1, Ordering::Relaxed);
        // Return pointer to the region after the header
        unsafe { NonNull::new_unchecked(raw.add(HEADER) as *mut c_void) }
    }

    unsafe fn deallocate_raw(&mut self, ptr: *mut c_void) {
        // Recover header
        let header = unsafe { (ptr as *mut u8).sub(HEADER) };
        let size = unsafe { (header as *mut usize).read() };
        let align = unsafe { (header as *mut usize).add(1).read() };
        let full_align = align.max(std::mem::align_of::<usize>());
        let full_size = HEADER + size;
        let layout = Layout::from_size_align(full_size, full_align).unwrap();
        unsafe { dealloc(header, layout) };
        self.live.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Wrapper that increments a counter when dropped.
/// Used to verify that `V::drop` is called the correct number of times.
#[allow(dead_code)]
pub struct DropCount<'a> {
    counter: &'a AtomicUsize,
    value: i32,
}

impl<'a> DropCount<'a> {
    #[allow(dead_code)]
    pub fn new(counter: &'a AtomicUsize, value: i32) -> Self {
        Self { counter, value }
    }
}

impl Drop for DropCount<'_> {
    fn drop(&mut self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
    }
}

impl PartialEq for DropCount<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for DropCount<'_> {}
impl PartialOrd for DropCount<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DropCount<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
