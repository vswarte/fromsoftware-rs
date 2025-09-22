use std::ffi;
use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::dlkr::DLPlainLightMutex;
use crate::fd4::{FD4BasicHashString, FD4FileCap, FD4ResCap, FD4ResCapHolder, FD4ResRep};
use crate::DoublyLinkedList;
use shared::OwnedPtr;

#[vtable_rs::vtable]
pub trait CSFileImpVmt {
    fn get_runtime_metadata(&self) -> usize;

    fn destructor(&mut self, param_2: u32);

    /// Retrieves a file cap from the primary file repository.
    fn get_file_cap(&mut self, name: &FD4BasicHashString) -> Option<NonNull<UntypedFileCap>>;

    /// Adds a file cap to the primary file repository. The file loading queue parameters indicates
    /// what file loading queue the load events will be handled by.
    fn add_file_cap(
        &mut self,
        name: &FD4BasicHashString,
        file_cap: &UntypedFileCap,
        file_loading_queue: u32,
    );

    fn unk_add_file_cap(
        &mut self,
        name: &FD4BasicHashString,
        file_cap: &UntypedFileCap,
        param_4: usize,
        param_5: usize,
        file_loading_queue: u32,
    );

    /// Removes the FileCap from the repositories, calls the destructor and deallocates the memory.
    /// This will often load to a crash-less unload of the resource but there are exceptions so be
    /// careful with calling this on specific resources.
    fn unload_file_cap_by_name(&mut self, name: &FD4BasicHashString);

    /// Unloads the referenced filecap.
    fn unload_file_cap(&mut self, file_cap: &UntypedFileCap);

    fn unk40(&mut self, file_cap: &UntypedFileCap);
}

/// Manages files used by the file, both virtual and on-disk.
#[repr(C)]
pub struct CSFileImp {
    vftable: VPtr<dyn CSFileImpVmt, Self>,
    pub file_repository_1: OwnedPtr<CSFileRepository>,
    // TODO: Incomplete..
}

/// Manages a set of files as well as keeps track of load state and such.
#[repr(C)]
pub struct CSFileRepository {
    pub res_rep: FD4ResRep<UntypedFileCap>,
    pub holder2: FD4ResCapHolder<UntypedFileCap>,
    unkc8: DoublyLinkedList<()>,
    pub mutexes: [OwnedPtr<CSFileRepositoryMutex>; 5],
    file_load_event_queues: [OwnedPtr<usize>; 5],
}

#[repr(C)]
pub struct CSFileRepositoryMutex {
    pub mutex: DLPlainLightMutex,
    unk30: u32,
    unk34: u32,
    unk38: u32,
    unk3c: u32,
    unk40: usize,
    unk48: usize,
}

#[repr(C)]
/// Used to represent file caps without a concrete type.
pub struct UntypedFileCap {
    pub file_cap: FD4FileCap<Self>,
}

impl AsRef<FD4ResCap<Self>> for UntypedFileCap {
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.file_cap.res_cap
    }
}
