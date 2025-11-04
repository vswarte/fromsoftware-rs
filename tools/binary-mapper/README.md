# Binary mapper

Tool to retrieve RVAs for functions and structures from the games binary.

To rebuild the `crates/game`'s rva.rs, invoke the following from the repo's root (make sure to replace`<game exe path>` with the appropriate path to your games exe):
`$ cargo run --bin binary-mapper -- --profile crates/util/mapper-profile.toml --exe <game exe path> --output rust > crates/util/src/rva.rs`

For steam on linux `<game exe path>` will probably be `~/.steam/steam/steamapps/common/ELDEN\ RING/Game/eldenring.exe`.

You can also pass `print` to the `--output` option to print the results to the CLI, this is useful when verifying if you it found the right RVA.

## Profile
The profile defines what the mapper is looking for and defines what RVAs to expose as a constant.

``` toml
[[patterns]]
pattern = "40 57 48 83 ec 40 48 c7 44 24 20 fe ff ff ff 48 89 5c 24 50 48 89 6c 24 58 48 89 74 24 60 49 8b f0 48 8b fa 48 8b d9 48 8d 69 28"
captures = ["CS_EZ_DRAW_DRAW_LINE"]
```

will look for the specified pattern and generates a constant for the RVA called `RVA_CS_EZ_DRAW_DRAW_LINE` in the output file. The start of the pattern will always be mapped to the first entry in the `captures` list.

### More complex patterns
Since this tool simply drives pelite's scanner the pattern itself offers a few utility features described [here](https://docs.rs/pelite/latest/x86_64-unknown-linux-gnu/pelite/pattern/fn.parse.html).

Tagging a specific part of the result with `'` will cause it to get mapped to the captures as a new entry. Having an empty string for a `captures` list item will ignore the result.

For example, the mapper config below will expose only the JMP target as `CS_WORLD_GEOM_MAN_BLOCK_DATA_BY_MAP_ID`.
```toml
[[patterns]]
pattern = "83 cb 02 89 5c 24 20 48 8d 54 24 38 e8 $ { ' }"
captures = ["", "CS_WORLD_GEOM_MAN_BLOCK_DATA_BY_MAP_ID"]
```

### RTTI Virtual Methods

Patterns can also be located using RTTI information embedded in the executable to find the addresses of virtual methods. For example:

```toml
[[vmts]]
class = "DLUID::MouseDevice<DLKR::DLMultiThreadingPolicy>"
captures = { "MOUSE_DEVICE_SHOULD_BLOCK_INPUT" = 27 }
```

The `class` field is the (unmangled) RTTI name of the class whose table to check, and `captures` is a map from capture names to the 0-based index of the virtual method being captured. Note that the resulting RVA points to the function itself, *not* its entry in the VMT.
