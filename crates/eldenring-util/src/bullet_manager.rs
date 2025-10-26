use eldenring::cs::{BulletSpawnData, CSBulletIns, CSBulletManager, FieldInsHandle};
use eldenring::rva;
use pelite::pe64::Pe;

use shared::program::Program;

pub trait CSBulletManagerExt {
    fn bullet_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut CSBulletIns>;

    fn spawn_bullet(&mut self, spawn_data: &BulletSpawnData) -> Result<(), i32>;
}

type FnSpawnBullet = extern "C" fn(&mut CSBulletManager, &mut i32, &BulletSpawnData, &mut i32);

impl CSBulletManagerExt for CSBulletManager {
    /// Retrieve a CSBulletIns by its FieldInsHandle.
    fn bullet_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut CSBulletIns> {
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
    fn spawn_bullet(&mut self, spawn_data: &BulletSpawnData) -> Result<(), i32> {
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
