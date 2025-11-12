pub mod arxan;
pub mod dl_math;
pub mod owned_pointer;
pub mod program;
pub mod rtti;
pub mod singleton;
pub mod task;

pub use arxan::*;
pub use dl_math::*;
pub use owned_pointer::*;
pub use program::*;
pub use rtti::*;
pub use singleton::*;
pub use task::*;

pub use fromsoftware_shared_macros::singleton;

extern crate self as fromsoftware_shared;
