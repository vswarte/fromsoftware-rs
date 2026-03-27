use std::ptr::NonNull;

use crate::Vector;

/// Base class that manages modifiers and converters.
#[repr(C)]
pub struct UserInputExtension {
    vftable: *const (),
}

/// Subclass of [UserInputExtension].
#[repr(C)]
pub struct DLUserInputPhysicalAnalogModifier {
    vftable: *const (),
}

/// Subclass of [UserInputExtension].
#[repr(C)]
pub struct DLUserInputDirectConverter {
    vftable: *const (),
    unk08: Vector<*const ()>,
    unk28: Vector<*const ()>,
    /// Points to field `unk08`.
    unk08_ptr: NonNull<Vector<*const ()>>,
    /// Points to field `unk28`.
    unk28_ptr: NonNull<Vector<*const ()>>,
}
