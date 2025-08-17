use eldenring::cs::{ChrIns, FieldInsHandle, WorldChrMan};
use shared::F32Vector4;

pub trait WorldChrManExt {
    fn chr_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut ChrIns>;
    fn spawn_debug_character(&mut self, request: &ChrDebugSpawnRequest);
}

impl WorldChrManExt for WorldChrMan {
    fn chr_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut ChrIns> {
        let chr_set_index = handle.selector.container() as usize;
        let chr_set = self.chr_sets.get_mut(chr_set_index)?.as_mut()?;

        chr_set.chr_ins_by_handle(handle)
    }

    fn spawn_debug_character(&mut self, request: &ChrDebugSpawnRequest) {
        let mut name_bytes = format!("c{:0>4}", request.chr_id)
            .encode_utf16()
            .collect::<Vec<_>>();

        name_bytes.resize(0x20, 0x0);

        self.debug_chr_creator
            .init_data
            .name
            .clone_from_slice(name_bytes.as_mut());

        self.debug_chr_creator.init_data.chara_init_param_id = request.chara_init_param_id;
        self.debug_chr_creator.init_data.npc_param_id = request.npc_param_id;
        self.debug_chr_creator.init_data.npc_think_param_id = request.npc_think_param_id;
        self.debug_chr_creator.init_data.event_entity_id = request.event_entity_id;
        self.debug_chr_creator.init_data.talk_id = request.talk_id;

        self.debug_chr_creator.init_data.spawn_position =
            F32Vector4(request.pos_x, request.pos_y, request.pos_z, 0.0);

        self.debug_chr_creator.spawn = true;
    }
}

pub struct ChrDebugSpawnRequest {
    pub chr_id: i32,
    pub chara_init_param_id: i32,
    pub npc_param_id: i32,
    pub npc_think_param_id: i32,
    pub event_entity_id: i32,
    pub talk_id: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}
