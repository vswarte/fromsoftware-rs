//! STL types. We can't directly use [cxx_stl] for these because it targets the
//! standardized type layouts that were introduced in MSVC 2018, and DS3 uses
//! MSVC 2015 instead.

pub mod vec;
