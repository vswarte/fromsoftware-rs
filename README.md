# FromSoftware-rs 🔩  From Software runtime rust bindings

Rust bindings to facilitate mod creation for From Software games.

[![Build Status](https://github.com/vswarte/eldenring-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/vswarte/eldenring-rs/actions)
![Crates.io License](https://img.shields.io/crates/l/eldenring)

## Examples
[Check out the examples directory](examples/README.md)

## Project structure (crates)

- `crates/eldenring` Contains the definitions for the elden ring structures. [![Crates.io](https://img.shields.io/crates/v/eldenring.svg?label=eldenring)](https://crates.io/crates/eldenring) [![Documentation](https://docs.rs/eldenring/badge.svg)](https://docs.rs/eldenring)
- `crates/nightreign` Contains the definitions for the nightreign structures. [![Crates.io](https://img.shields.io/crates/v/nightreign.svg?label=nightreign)](https://crates.io/crates/nightreign) [![Documentation](https://docs.rs/nightreign/badge.svg)](https://docs.rs/nightreign)
- `crates/util` Provides helper methods for common stuff. [![Crates.io](https://img.shields.io/crates/v/eldenring-util.svg?label=eldenring-util)](https://crates.io/crates/eldenring-util) [![Documentation](https://docs.rs/eldenring-util/badge.svg)](https://docs.rs/eldenring-util)
- `crates/dlrf` Defines a trait and exports a macro for interacting with the games reflection system. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf.svg?label=eldenring-dlrf)](https://crates.io/crates/eldenring-dlrf)  [![Documentation](https://docs.rs/eldenring-dlrf/badge.svg)](https://docs.rs/eldenring-dlrf)
- `crates/dlrf/derive` Defines the derive macro for implementing the DLRF trait on types. **Do not depend on this directly since the macro is reexported through `eldenring-dlrf`**. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf-derive.svg?label=eldenring-dlrf-derive)](https://crates.io/crates/eldenring-dlrf-derive)  [![Documentation](https://docs.rs/eldenring-dlrf-derive/badge.svg)](https://docs.rs/eldenring-dlrf-derive)

## Credits (aside listed contributors to this repository)

- Tremwil (for the arxan code restoration disabler, vtable-rs and a few other boilerplate-y things as well as implementing the initial FD4 singleton finder for TGA that I appropriated).
- Dasaav (for [libER](https://github.com/Dasaav-dsv/libER) and heaps of engine-related structures).
- Sfix (for coming up with the FD4 singleton finder approach at all).
- Yui (for some structures as well as AOBs and hinting at some logic existing in the binary).
- Vawser (and probably many more) (for hosting the param defs used with the param struct generator).

(Have you contributed to TGA in some manner and does this repository have your work in it? Reach out to @chainfailure on Discord for proper credit disclosure).
