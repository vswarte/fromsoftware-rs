use std::fmt::Debug;

/// A trait that's intended to be applied to a type using a derive macro when
/// actively trying to research the behavior and meaning of that type's fields.
///
/// This should never be used in checked-in code. It exists only as a
/// convenience during active reverse-engineering.
pub trait Researching {
    /// Returns a list of all unknown field names and references to their values.
    ///
    /// When `#[derive(Researching)]` is used, it treats and field whose name
    /// begins with `_unk` or `unk` as unknown.
    fn unknown_fields(&self) -> Vec<(&str, &dyn Debug)>;
}
