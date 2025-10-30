# Param Generator

Generates rust structs from specified paramdef XML files.
This tool ignores any paramdef fields with a maximum version specified since we assume we're always building for the latest version of the game.

To rebuild `crates/eldenring`'s param.rs, invoke the following from the repo's root:
`$ cargo run --bin param-generator -- --input tools/param-generator/params/eldenring --output crates/eldenring/src/param.rs`

To rebuild `crates/nightreign`'s param.rs, invoke the following from the repo's root:
`$ cargo run --bin param-generator -- --input tools/param-generator/params/nightreign --output crates/nightreign/src/param.rs`

To rebuild `crates/darksouls3`'s param.rs, invoke the following from the repo's root:
`$ cargo run --bin param-generator -- --input tools/param-generator/params/darksouls3 --output crates/darksouls3/src/param.rs`

Make sure to run a round of rustfmt on the output, otherwise the output will not comply with the enforced styleguide.
