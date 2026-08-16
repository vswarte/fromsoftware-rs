use std::{borrow::Cow, ptr::NonNull};

use shared::{AllocError, FromStatic, GameAllocator, InstanceResult, load_static_indirect};

use crate::dlkr::{DLAllocator, DLHeapIdentifier, PlainAdaptiveMutexImpl};

#[repr(C)]
pub struct HeapAllocator<T> {
    pub base: DLAllocator,
    pub heap: NonNull<T>,
}

impl<T> std::ops::Deref for HeapAllocator<T> {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub struct MainHeap;

pub type MainHeapAllocator = HeapAllocator<MainHeap>;

impl FromStatic for MainHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("MainHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().main_heap_allocator) }
    }
}

pub(crate) fn heap_allocate<T>(layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError>
where
    T: FromStatic + std::ops::Deref<Target = DLAllocator> + 'static,
{
    let allocator = unsafe { T::instance() }.map_err(|_| AllocError)?;

    let ptr = (allocator.vftable.allocate_aligned)(allocator, layout.size(), layout.align());
    let ptr = NonNull::new(ptr as *mut u8).ok_or(AllocError)?;
    Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
}

pub(crate) unsafe fn heap_deallocate<T>(ptr: NonNull<u8>)
where
    T: FromStatic + std::ops::Deref<Target = DLAllocator> + 'static,
{
    let Ok(allocator) = (unsafe { T::instance() }) else {
        return;
    };

    (allocator.vftable.deallocate)(allocator, ptr.as_ptr().cast())
}

impl<T: 'static> GameAllocator for HeapAllocator<T>
where
    HeapAllocator<T>: FromStatic,
{
    fn allocate(layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        heap_allocate::<Self>(layout)
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe { heap_deallocate::<Self>(ptr) }
    }
}

/// Wrapper around [`MainHeapAllocator`] that redirects all allocations to [`super::DLAllocatorVmt::back_allocate`] instead of [`super::DLAllocatorVmt::allocate`].
#[repr(C)]
pub struct DLBackAllocator {
    pub base: DLAllocator,
    pub underlying: &'static MainHeapAllocator,
}

impl std::ops::Deref for DLBackAllocator {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl GameAllocator for DLBackAllocator {
    fn allocate(layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        heap_allocate::<Self>(layout)
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe { heap_deallocate::<Self>(ptr) }
    }
}

impl FromStatic for DLBackAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DLBackAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().dl_back_allocator) }
    }
}

pub struct GfxHeap;

/// The graphics heap allocator (`GFX` heap).
pub type GfxHeapAllocator = HeapAllocator<GfxHeap>;

impl FromStatic for GfxHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_heap_allocator) }
    }
}

pub struct GfxTempHeap;
/// The temporary graphics heap allocator (`GFXTEMP` heap).
pub type GfxTempHeapAllocator = HeapAllocator<GfxTempHeap>;

impl FromStatic for GfxTempHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxTempHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_temp_heap_allocator) }
    }
}

pub struct InGameHeap;
/// The in-game heap allocator (`INGAME` heap).
pub type InGameHeapAllocator = HeapAllocator<InGameHeap>;

impl FromStatic for InGameHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("InGameHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().ingame_heap_allocator) }
    }
}

pub struct TempHeap;
/// The general temporary heap allocator (`TEMP` heap).
pub type TempHeapAllocator = HeapAllocator<TempHeap>;

impl FromStatic for TempHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("TempHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().temp_heap_allocator) }
    }
}

pub struct CoreResHeap;
/// The core resource heap allocator (`CORERES` heap).
pub type CoreResHeapAllocator = HeapAllocator<CoreResHeap>;

impl FromStatic for CoreResHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CoreResHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().core_res_heap_allocator) }
    }
}

pub struct MoWwiseHeap;
/// The WWise audio engine's heap allocator (`MO_WWISE` heap).
pub type MoWwiseHeapAllocator = HeapAllocator<MoWwiseHeap>;

impl FromStatic for MoWwiseHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("MoWwiseHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().mo_wwise_heap_allocator) }
    }
}

/// A `CSMoWwiseAllocator` wrapping [`MoWwiseHeapAllocator`]
/// (`MO_WWISE_CUSTOM` heap).
#[repr(C)]
pub struct CSMoWwiseAllocator {
    pub base: DLAllocator,
    pub underlying: &'static MoWwiseHeapAllocator,
    /// The threshold at which the allocator will use [`super::DLAllocatorVmt::back_allocate`] instead of [`super::DLAllocatorVmt::allocate`].
    pub back_allocate_threshold: usize,
}

impl std::ops::Deref for CSMoWwiseAllocator {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl FromStatic for CSMoWwiseAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CSMoWwiseAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().cs_mo_wwise_allocator) }
    }
}

pub struct MoWwiseMoOnlyHeap;
/// The WWise "MO only" heap allocator (`MO_WWISE_MO_ONLY` heap).
pub type MoWwiseMoOnlyHeapAllocator = HeapAllocator<MoWwiseMoOnlyHeap>;

impl FromStatic for MoWwiseMoOnlyHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("MoWwiseMoOnlyHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().mo_wwise_mo_only_heap_allocator) }
    }
}

pub struct MoWwiseIsorationHeap;
/// The WWise isolation heap allocator (`MO_WWISE_ISORATION` heap).
///
/// Typo is in the original name from RTTI.
pub type MoWwiseIsorationHeapAllocator = HeapAllocator<MoWwiseIsorationHeap>;

impl FromStatic for MoWwiseIsorationHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("MoWwiseIsorationHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().mo_wwise_isoration_heap_allocator) }
    }
}

pub struct LuaHeap;
/// The Lua scripting heap allocator (`LUA` heap).
pub type LuaHeapAllocator = HeapAllocator<LuaHeap>;

impl FromStatic for LuaHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("LuaHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().lua_heap_allocator) }
    }
}

pub struct HavokHeap;
/// The Havok physics engine's heap allocator (`HAVOK` heap).
pub type HavokHeapAllocator = HeapAllocator<HavokHeap>;

impl FromStatic for HavokHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("HavokHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().havok_heap_allocator) }
    }
}

pub struct MenuHeap;
/// The menu/UI heap allocator (`MENU`).
pub type MenuHeapAllocator = HeapAllocator<MenuHeap>;

impl FromStatic for MenuHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("MenuHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().menu_heap_allocator) }
    }
}

#[repr(C)]
pub struct CSNetworkAllocator {
    pub base: DLAllocator,
    pub underlying: &'static DLAllocator,
}

impl FromStatic for CSNetworkAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CSNetworkAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().network_heap_allocator) }
    }
}

impl std::ops::Deref for CSNetworkAllocator {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl GameAllocator for CSNetworkAllocator {
    fn allocate(layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        heap_allocate::<Self>(layout)
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe { heap_deallocate::<Self>(ptr) }
    }
}

pub struct DebugHeap;
/// The debug heap allocator (`DEBUG` heap).
pub type DebugHeapAllocator = HeapAllocator<DebugHeap>;

impl FromStatic for DebugHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DebugHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().debug_heap_allocator) }
    }
}

pub struct GfxSystemSharedHeap;
/// The shared graphics-system heap allocator (`GFX_SystemShared` heap).
pub type GfxSystemSharedHeapAllocator = HeapAllocator<GfxSystemSharedHeap>;

impl FromStatic for GfxSystemSharedHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxSystemSharedHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_system_shared_heap_allocator) }
    }
}

pub struct GfxGraphicsPrivateAHeap;
/// Graphics private heap allocator A (`GFX_GraphicsPrivate_A` heap).
pub type GfxGraphicsPrivateAHeapAllocator = HeapAllocator<GfxGraphicsPrivateAHeap>;

impl FromStatic for GfxGraphicsPrivateAHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxGraphicsPrivateAHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_graphics_private_a_heap_allocator) }
    }
}

pub struct GfxGraphicsPrivateBHeap;
/// Graphics private heap allocator B (`GFX_GraphicsPrivate_B` heap).
pub type GfxGraphicsPrivateBHeapAllocator = HeapAllocator<GfxGraphicsPrivateBHeap>;

impl FromStatic for GfxGraphicsPrivateBHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxGraphicsPrivateBHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_graphics_private_b_heap_allocator) }
    }
}

/// A `CSGraphicsPrivateAllocator` mixing [`GfxGraphicsPrivateAHeapAllocator`]
/// and [`GfxGraphicsPrivateBHeapAllocator`] (`GFX_GraphicsPrivateMix` heap).
#[repr(C)]
pub struct GfxGraphicsPrivateMixHeapAllocator {
    pub base: DLAllocator,
    /// Allocator for requests of 1 MiB (0x100000) or more.
    pub private_a: &'static GfxGraphicsPrivateAHeapAllocator,
    /// Allocator for requests smaller than 1 MiB (0x100000).
    pub private_b: &'static GfxGraphicsPrivateBHeapAllocator,
    pub sync: PlainAdaptiveMutexImpl,
    pub heap_id: DLHeapIdentifier,
}

impl std::ops::Deref for GfxGraphicsPrivateMixHeapAllocator {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl FromStatic for GfxGraphicsPrivateMixHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GfxGraphicsPrivateMixHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().gfx_graphics_private_mix_heap_allocator) }
    }
}

pub struct RSResourceManagerHeap;

pub type RSResourceManagerHeapAllocator = HeapAllocator<RSResourceManagerHeap>;

impl FromStatic for RSResourceManagerHeapAllocator {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("RSResourceManagerHeapAllocator")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().rs_resource_manager_heap_allocator) }
    }
}
