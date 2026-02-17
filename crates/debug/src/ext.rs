use std::fmt::{Debug, Display, Pointer};

use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

/// Extension trait for UI helpers.
pub trait UiExt {
    /// Renders a key/value text widget with the name `label` and the value
    /// given by `value`'s [Debug] format.
    ///
    /// Shorthand for `ui.debug(format!("{}: {:?}"), label, value)`.
    fn debug(&self, label: impl AsRef<str>, value: impl Debug);

    /// Renders a key/value text widget with the name `label` and the value
    /// given by `value`'s [Display] format.
    ///
    /// Shorthand for `ui.debug(format!("{}: {}"), label, value)`.
    fn display(&self, label: impl AsRef<str>, value: impl Display);

    /// Renders a key/value text widget with the name `label` and the value
    /// given by `value`'s [Debug] format. The value is in a read-only text
    /// field so it can be copy/pasted.
    fn debug_copiable(&self, label: impl AsRef<str>, value: impl Debug);

    /// Renders a key/value text widget with the name `label` and the value
    /// given by `value`'s [Display] format. The value is in a read-only text
    /// field so it can be copy/pasted.
    fn display_copiable(&self, label: impl AsRef<str>, value: impl Display);

    /// Renders a key/value text widget with the name `label` and the value
    /// given by `value`'s [Pointer] format. The value is in a read-only text
    /// field so it can be copy/pasted.
    fn pointer(&self, label: impl AsRef<str>, value: impl Pointer);

    /// Renders a collapsing header with the given `label` and the contents of
    /// `f` indented beneath it.
    ///
    /// This automatically creates a unique ID for the header so that its
    /// collapsed state doesn't collide with other headers.
    fn header(&self, label: impl AsRef<str>, f: impl FnOnce());

    /// If `value` isn't `None`, renders a collapsing header with the given
    /// `label` and the contents of `f` indented beneath it. Otherwise, renders
    /// a text line indicating that `value` is `None`.
    ///
    /// This automatically creates a unique ID for the header so that its
    /// collapsed state doesn't collide with other headers.
    fn header_opt<T>(&self, label: impl AsRef<str>, value: Option<T>, f: impl FnOnce(T));

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
        Name: std::convert::AsRef<str>;
}

impl UiExt for Ui {
    fn debug(&self, label: impl AsRef<str>, value: impl Debug) {
        self.text(format!("{}: {:?}", label.as_ref(), value));
    }

    fn display(&self, label: impl AsRef<str>, value: impl Display) {
        self.text(format!("{}: {}", label.as_ref(), value));
    }

    fn debug_copiable(&self, label: impl AsRef<str>, value: impl Debug) {
        self.display_copiable(label, format!("{:?}", value));
    }

    fn display_copiable(&self, label: impl AsRef<str>, value: impl Display) {
        self.text(format!("{}:", label.as_ref()));
        self.same_line();

        // Don't use the built-in label because it appears after the text.
        let _id = self.push_id(label);
        self.input_text("", &mut value.to_string())
            .read_only(true)
            .build();
    }

    fn pointer(&self, label: impl AsRef<str>, value: impl Pointer) {
        self.display_copiable(label, format!("{:p}", value));
    }

    fn header(&self, label: impl AsRef<str>, f: impl FnOnce()) {
        let label = label.as_ref();
        let _id = self.push_id(label);
        if self.collapsing_header(label, TreeNodeFlags::empty()) {
            self.indent();
            f();
            self.unindent();
        }
    }

    fn header_opt<T>(&self, label: impl AsRef<str>, value: Option<T>, f: impl FnOnce(T)) {
        if let Some(value) = value {
            self.header(label, || f(value));
        } else {
            self.text(format!("{}: None", label.as_ref()));
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
