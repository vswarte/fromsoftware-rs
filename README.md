# FromSoftware-rs 🔩  From Software runtime rust bindings

Rust bindings to facilitate mod creation for From Software games.

[![Build Status](https://github.com/vswarte/eldenring-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/vswarte/eldenring-rs/actions)
![Crates.io License](https://img.shields.io/crates/l/eldenring)

## Examples
[Check out the examples directory](examples/README.md)

## Project structure (crates)

- `crates/darksouls3` Contains the definitions for the Dark Souls 3 structures. [![Crates.io](https://img.shields.io/crates/v/darksouls3.svg?label=darksouls3)](https://crates.io/crates/darksouls3) [![Documentation](https://docs.rs/darksouls3/badge.svg)](https://docs.rs/darksouls3)
- `crates/eldenring` Contains the definitions for the Elden Ring structures. [![Crates.io](https://img.shields.io/crates/v/eldenring.svg?label=eldenring)](https://crates.io/crates/eldenring) [![Documentation](https://docs.rs/eldenring/badge.svg)](https://docs.rs/eldenring)
- `crates/nightreign` Contains the definitions for the Elden Ring: Nightreign structures. [![Crates.io](https://img.shields.io/crates/v/nightreign.svg?label=nightreign)](https://crates.io/crates/nightreign) [![Documentation](https://docs.rs/nightreign/badge.svg)](https://docs.rs/nightreign)
- `crates/shared` Defines structures and utilities that are shared across multiple From Software games. [![Crates.io](https://img.shields.io/crates/v/fromsoftware-shared.svg?label=shared)](https://crates.io/crates/fromsoftware-shared)  [![Documentation](https://docs.rs/fromsoftware-shared/badge.svg)](https://docs.rs/fromsoftware-shared)
- `crates/shared/macros` Defines a derive macro for implementing the `FromSingleton` trait on types. **Do not depend on this directly since the macro is reexported through `fromsoftware-shared`**. [![Crates.io](https://img.shields.io/crates/v/fromsoftware-shared-macros.svg?label=fromsoftware-shared-macros)](https://crates.io/crates/fromsoftware-shared-macros)  [![Documentation](https://docs.rs/fromsoftware-shared-macros/badge.svg)](https://docs.rs/fromsoftware-shared-macros)

## Credits (aside listed contributors to this repository)

- Tremwil (for the arxan code restoration disabler, vtable-rs and a few other boilerplate-y things as well as implementing the initial FD4 singleton finder for TGA that I appropriated).
- Dasaav (for [libER](https://github.com/Dasaav-dsv/libER) as well as [from-singleton](https://github.com/Dasaav-dsv/from-singleton) and heaps of engine-related structures).
- Sfix (for coming up with the FD4 singleton finder approach at all).
- Yui (for the arxan code restoration disabler as well as some structures and AOBs).
- Vawser (and probably many more) (for hosting the param defs used with the param struct generator).

(Have you contributed to TGA in some manner and does this repository have your work in it? Reach out to @chainfailure on Discord for proper credit disclosure).
