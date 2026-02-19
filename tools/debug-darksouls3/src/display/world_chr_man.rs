use hudhook::imgui::Ui;

use darksouls3::sprj::*;
use debug::{StateMap, UiExt};
use fromsoftware_shared::Subclass;

use super::{StatefulDebugDisplay, chr::ChrInsState, world_block::WorldBlockChrState};

#[derive(Default)]
pub struct WorldChrManState {
    player_chr_set_state: ChrSetState,
    ghost_chr_set_state: ChrSetState,
    debug_chr_set_state: ChrSetState,
    main_player_state: ChrInsState,
    world_block_chr_states: StateMap<BlockId, WorldBlockChrState>,
}

impl StatefulDebugDisplay for WorldChrMan {
    type State = WorldChrManState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        state.world_block_chr_states.track_reads();
        ui.debug("World Area Chr Len", self.world_area_chr_len);

        let mut world_block_chrs = self.block_chrs_mut().collect::<Vec<_>>();
        ui.list(
            format!("World Block Chrs: {}", world_block_chrs.len()),
            world_block_chrs.iter_mut(),
            |ui, i, world_block_chr| {
                ui.header(format!("Block {}", i), || {
                    let state = state
                        .world_block_chr_states
                        .get(world_block_chr.info().block_id);
                    world_block_chr.render_debug_mut(ui, state);
                });
            },
        );

        ui.debug("World Block Chr Count", self.world_block_chr_count);
        ui.debug(
            "Loaded? World Block Chr Count",
            self.loaded_world_block_chr_count,
        );

        ui.header("Player ChrSet", || {
            self.player_chr_set
                .render_debug_mut(ui, &mut state.player_chr_set_state);
        });

        ui.header("Ghost ChrSet", || {
            self.ghost_chr_set
                .render_debug_mut(ui, &mut state.ghost_chr_set_state);
        });

        ui.header("Debug ChrSet", || {
            self.debug_chr_set
                .render_debug_mut(ui, &mut state.debug_chr_set_state);
        });

        ui.header_opt("Main player", self.main_player.as_mut(), |p| {
            unsafe { p.as_mut() }.render_debug_mut(ui, &mut state.main_player_state);
        });

        state.world_block_chr_states.remove_unread();
    }
}

#[derive(Default)]
pub struct ChrSetState {
    chr_ins_states: StateMap<u32, ChrInsState>,
}

impl<T> StatefulDebugDisplay for ChrSet<T>
where
    T: Subclass<ChrIns>,
{
    type State = ChrSetState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        state.chr_ins_states.track_reads();
        let characters = self.iter_mut().collect::<Vec<_>>();
        ui.list(
            format!("Characters: {}", characters.len()),
            characters,
            |ui, _, entry| {
                let chr_ins = entry.chr.as_mut();
                ui.header(format!("{} ##{:p}", chr_ins.id(), chr_ins), || {
                    let sup = chr_ins.superclass_mut();
                    let state = state.chr_ins_states.get(sup.field_ins_handle);
                    sup.render_debug_mut(ui, state);
                });
            },
        );
        state.chr_ins_states.remove_unread();
    }
}
