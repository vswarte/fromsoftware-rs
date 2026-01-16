//! This module contains higher-level utilities for working with Dark Souls III.
//! They're built on top of the definitions in the rest of the crate, and
//! they're intended to simplify common mod tasks and provide a safer, more
//! Rust-like interface.
//!
//! ## Compatibility
//!
//! This module's APIs make assumptions about how the game's objects operate,
//! and in some cases even call out to original game functions. They're tested
//! against supported game versions and every effort is made to ensure they'll
//! be compatible, but predicting what future patches will change is impossible
//! so there's a higher risk that these APIs will break when new patches are
//! released.

pub mod system;
