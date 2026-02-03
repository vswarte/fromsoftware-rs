use fromsoftware_shared::FromStatic;
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

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
pub(crate) mod menu_man;
pub(crate) mod net_man;
pub(crate) mod param;
pub(crate) mod session_manager;
pub(crate) mod sfx;
pub(crate) mod shared;
pub(crate) mod task;
pub(crate) mod world_chr_man;

pub trait DebugDisplay {
    fn render_debug(&self, ui: &Ui);
}

/// Extension trait for UI helpers
pub trait UiExt {
    fn header(&self, label: &str, f: impl FnOnce());

    fn list<I, T>(&self, label: &str, items: I, render_item: impl FnMut(&Self, usize, T))
    where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>;

    fn table<I, T, Name, const N: usize>(
        &self,
        label: &str,
        columns: [TableColumnSetup<Name>; N],
        items: I,
        render_row: impl FnMut(&Self, usize, T),
    ) where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>,
        Name: std::convert::AsRef<str>;
}

impl UiExt for Ui {
    fn header(&self, label: &str, f: impl FnOnce()) {
        let _id = self.push_id(label);
        if self.collapsing_header(label, TreeNodeFlags::empty()) {
            self.indent();
            f();
            self.unindent();
        }
    }

    fn list<I, T>(&self, label: &str, items: I, mut render_item: impl FnMut(&Self, usize, T))
    where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>,
    {
        self.header(label, || {
            for (i, item) in items.into_iter().enumerate() {
                let _id = self.push_id_usize(i);
                render_item(self, i, item);
            }
        });
    }

    fn table<I, T, Name, const N: usize>(
        &self,
        label: &str,
        columns: [TableColumnSetup<Name>; N],
        items: I,
        mut render_row: impl FnMut(&Self, usize, T),
    ) where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>,
        Name: std::convert::AsRef<str>,
    {
        if let Some(_t) = self.begin_table_header_with_flags(
            label,
            columns,
            TableFlags::RESIZABLE
                | TableFlags::BORDERS
                | TableFlags::ROW_BG
                | TableFlags::SIZING_STRETCH_PROP,
        ) {
            for (i, item) in items.into_iter().enumerate() {
                let _id = self.push_id_usize(i);
                render_row(self, i, item);
            }
        }
    }
}

pub fn render_debug_static<T: FromStatic + DebugDisplay + 'static>(ui: &Ui) {
    let instance = unsafe { T::instance() };

    match instance {
        Ok(instance) => {
            ui.header(&T::name(), || {
                let pointer = instance as *const T;
                let mut pointer_string = format!("{pointer:#x?}");
                let label = format!("{} instance", T::name());
                ui.input_text(label.as_str(), &mut pointer_string)
                    .read_only(true)
                    .build();

                instance.render_debug(ui);
            });
            ui.separator();
        }
        Err(err) => ui.text(format!("Couldn't load {}: {:?}", T::name(), err)),
    }
}
