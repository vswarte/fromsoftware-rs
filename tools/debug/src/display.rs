use fromsoftware_shared::{FromStatic, FromSingleton};
use hudhook::imgui::{TreeNodeFlags, Ui};

pub(crate) mod area_time;
pub(crate) mod auto_invade_point;
pub(crate) mod bullet_manager;
pub(crate) mod camera;
pub(crate) mod chr;
pub(crate) mod dlio;
pub(crate) mod event_flag;
pub(crate) mod event_man;
pub(crate) mod fade;
pub(crate) mod fe_man;
pub(crate) mod field_area;
pub(crate) mod gaitem;
pub(crate) mod geometry;
pub(crate) mod gparam;
pub(crate) mod net_man;
pub(crate) mod param;
pub(crate) mod session_manager;
pub(crate) mod sfx;
pub(crate) mod shared;
pub(crate) mod task;
pub(crate) mod world_chr_man;

pub trait DebugDisplay {
    fn render_debug(&self, ui: &&mut Ui);
}

pub fn render_debug_singleton<T: FromSingleton + DebugDisplay + 'static>(ui: &&mut Ui) {
    let singleton = unsafe { T::instance() };

    match singleton {
        Ok(instance) => {
            if ui.collapsing_header(T::name(), TreeNodeFlags::empty()) {
                ui.indent();
                let pointer = instance as *const T;
                let mut pointer_string = format!("{pointer:#x?}");
                let label = format!("{} instance", T::name());
                ui.input_text(label.as_str(), &mut pointer_string)
                    .read_only(true)
                    .build();

                instance.render_debug(ui);
                ui.unindent();
                ui.separator();
            }
        }
        Err(err) => ui.text(format!("Couldn't load {}: {:?}", T::name(), err)),
    }
}
