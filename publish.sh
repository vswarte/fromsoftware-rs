#!/bin/bash
cargo publish --allow-dirty --no-verify -p fromsoftware-dlrf-derive
cargo publish --allow-dirty --no-verify -p fromsoftware-dlrf
cargo publish --allow-dirty --no-verify -p fromsoftware-shared
cargo publish --allow-dirty --no-verify -p eldenring
cargo publish --allow-dirty --no-verify -p eldenring-util
