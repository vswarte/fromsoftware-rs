use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

/// Extension trait for UI helpers.
pub trait UiExt {
    /// Renders a collapsing header with the given `label` and the contents of
    /// `f` indented beneath it.
    ///
    /// This automatically creates a unique ID for the header so that its
    /// collapsed state doesn't collide with other headers.
    fn header(&self, label: impl AsRef<str>, f: impl FnOnce());

    /// Renders a collapsing header with the given `label` and a sequence of
    /// items nested beneath it.
    ///
    /// `render_item` is called once for each item in `items`, along with that
    /// item's (0-based) index.
    fn list<I, T>(
        &self,
        label: impl AsRef<str>,
        items: I,
        render_item: impl FnMut(&Self, usize, T),
    ) where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>;

    /// Renders a collapsing header with the given `label` and a table nested
    /// beneath it.
    ///
    /// The table's column names are given by `columns` and its rows are given
    /// by `items`. Each row is rendered by calling `render_row` with the index
    /// of a `T` in `items`.
    fn table<I, T, Name, const N: usize>(
        &self,
        label: impl AsRef<str>,
        columns: [TableColumnSetup<Name>; N],
        items: I,
        render_row: impl FnMut(&Self, usize, T),
    ) where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>,
        Name: std::convert::AsRef<str>;
}

impl UiExt for Ui {
    fn header(&self, label: impl AsRef<str>, f: impl FnOnce()) {
        let label = label.as_ref();
        let _id = self.push_id(label);
        if self.collapsing_header(label, TreeNodeFlags::empty()) {
            self.indent();
            f();
            self.unindent();
        }
    }

    fn list<I, T>(
        &self,
        label: impl AsRef<str>,
        items: I,
        mut render_item: impl FnMut(&Self, usize, T),
    ) where
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
        label: impl AsRef<str>,
        columns: [TableColumnSetup<Name>; N],
        items: I,
        mut render_row: impl FnMut(&Self, usize, T),
    ) where
        I: IntoIterator<Item = T>,
        T: std::borrow::Borrow<T>,
        Name: AsRef<str>,
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
