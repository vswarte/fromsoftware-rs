use pelite::pe64::Pe;
use std::ptr::NonNull;

use shared::program::Program;
use shared::{F32Vector4, OwnedPtr};

use super::{CSBulletIns, ChrCam, FieldInsHandle};
use crate::rva;

#[repr(C)]
#[shared::singleton("CSBulletManager")]
/// Manages bullets...
///
/// Source of name: DLRF RuntimeClass metadata
pub struct CSBulletManager {
    pub bullets: BufferAndAllocLinkedList<CSBulletIns, 128, 128>,
    unk_bullet_sfx_related: BufferAndAllocLinkedList<[u8; 0x9d0], 64, 192>,
    unk40: BufferAndAllocLinkedList<[u8; 0x4220], 4, 28>,
    pub chr_cam: Option<NonNull<ChrCam>>,
    unk68: u8,
    _pad69: [u8; 7],
    unk70: u64,
    net_packing_vector_bullet_sync_setting: [u8; 0x28],
    net_packing_vector_bullet_emitter_destroy_sync_info: [u8; 0x28],
    net_packing_vector_bullet_inisync_setting: [u8; 0x28],
    net_packing_vector_change_target_req1: [u8; 0x28],
    net_packing_vector_change_target_req2: [u8; 0x28],
    net_packing_vector_bullet_on_hit_sync_info: [u8; 0x28],
    unk168: [u8; 0x18],
    unk180: [u8; 0x20],
    unk1a0: u8,
    _pad1a1: [u8; 7],
    unk1a8_func_table: usize,
    unk1b0_func_table: usize,
    unk1b8_func_table: usize,
    unk1c0: u8,
    _pad1c1: [u8; 3],
    unk1c4: f32,
    unk1c8: f32,
    unk1cc: u32,
    sfx_related_in_buffer_count: u32,
    sfx_related_count: u32,
    bullets_in_buffer_count: u32,
    bullets_count: u32,
    unk1e0: u32,
    unk1e4: u32,
    debug_draw_1e8: bool,
    debug_draw_1e9: bool,
    debug_draw_1ea: bool,
    debug_draw_1eb: bool,
    debug_draw_1ec: bool,
    debug_draw_1ed: bool,
    debug_draw_1ee: bool,
    debug_draw_1ef: bool,
    debug_draw_1f0: bool,
    _pad1f1: [u8; 3],
    unk1f4: f32,
    unk1f8: f32,
    _pad1fc: [u8; 4],
    unk200: [f32; 4],
    unk210: [f32; 4],
    unk220: [f32; 4],
    unk230: [f32; 4],
    unk240: [f32; 4],
    unk250: [u8; 0x20],
    debug_draw_270: bool,
    _pad271: [u8; 3],
    unk274: f32,
    unk278: usize,
}

type FnSpawnBullet = extern "C" fn(&mut CSBulletManager, &mut i32, &BulletSpawnData, &mut i32);

impl CSBulletManager {
    pub fn bullets(&self) -> impl Iterator<Item = &CSBulletIns> {
        let mut current = self.bullets.head;

        std::iter::from_fn(move || {
            current.map(|n| unsafe {
                let n = n.as_ref();
                current = n.next_bullet;
                n
            })
        })
    }

    pub fn bullets_mut(&mut self) -> impl Iterator<Item = &mut CSBulletIns> {
        let mut current = self.bullets.head;

        std::iter::from_fn(move || {
            current.map(|mut n| unsafe {
                let n = n.as_mut();
                current = n.next_bullet;
                n
            })
        })
    }

    /// Retrieve a CSBulletIns by its FieldInsHandle.
    pub fn bullet_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut CSBulletIns> {
        let index = handle.selector.0 & 0xFF;

        // In buffer, array access
        if index < 0x80 {
            let bullet = &mut self.bullets.prealloc_buffer[index as usize];

            if bullet.field_ins_handle == *handle {
                return Some(bullet);
            }
        }
        // Not in buffer, iterate linked list
        else if index == 254 {
            let mut current = &self.bullets.head;
            while !current.is_none() {
                let bullet = unsafe { current.unwrap().as_mut() };

                if bullet.field_ins_handle == *handle {
                    return Some(bullet);
                }

                current = &bullet.next_bullet;
            }
        }

        None
    }

    /// Spawns a single bullet from supplied spawn data.
    ///
    /// Error codes seem to be:
    /// - 0: Unknown
    /// - 3: Invalid bullet ID
    /// - 4: Construction failed
    pub fn spawn_bullet(&mut self, spawn_data: &BulletSpawnData) -> Result<(), i32> {
        let target = unsafe {
            std::mem::transmute::<u64, FnSpawnBullet>(
                Program::current()
                    .rva_to_va(rva::get().cs_bullet_manager_spawn_bullet)
                    .unwrap(),
            )
        };

        let unk_out = &mut 0;
        let error_out = &mut 0;
        target(self, unk_out, spawn_data, error_out);

        if *unk_out == -1 {
            return Err(*error_out);
        }

        Ok(())
    }
}

#[repr(C)]
/// Contains a pre allocated buffer that takes priority when creating a new T, when full
/// starts allocating manually on the heap.
/// Living elements create a linked list.
pub struct BufferAndAllocLinkedList<T, const BUFFER_SIZE: usize, const MAX_ALLOCS: usize> {
    pub prealloc_buffer: OwnedPtr<[T; BUFFER_SIZE]>,
    pub head: Option<NonNull<T>>,
    /// If buffer is full, None
    empty_spot: Option<NonNull<T>>,
    /// Amount of allocated Ts alive
    allocated_count: u32,
    /// Doesn't decrease
    allocations_counter: u32,
}

#[repr(C)]
pub struct BulletSpawnData {
    owner: FieldInsHandle,
    behavior_id: i32,
    magic_id: i32,
    unk10: u32,
    bullet_id: i32,
    goods_id: i32,
    dummy_poly_id: i32,
    /// Replaces owner's target if not -1
    target: FieldInsHandle,
    unk28: u32,
    unk2c: u32,
    unk30: F32Vector4,
    unk40: u32,
    unk44: u32,
    pad48: [u8; 8],
    /// Forward vector, only applies if angle vec is 0?
    acceleration_angle: F32Vector4,
    unk60: F32Vector4,
    /// Forward vector
    angle: F32Vector4,
    position: F32Vector4,
    unk90: u64,
    unk98: u64,
    unka0: u64,
    pada8: [u8; 8],
    unkb0_struct: [u8; 0x50],
    unk100: u8,
    pad101: [u8; 15],
}

#[cfg(test)]
mod test {
    use crate::cs::CSBulletManager;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x280, size_of::<CSBulletManager>());
    }
}
