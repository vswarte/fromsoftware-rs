# Binary mapper

Tool to retrieve RVAs for functions and structures from the games binary.

In most cases, you can run the shortcut mode for whichever game needs its RVAs updated. For Elden Ring:

```
$ cargo run --bin binary-mapper -- er --ww-exe "<game exe path>" --jp-exe "<game exe path>"
```

For steam on linux `<game exe path>` will probably be `~/.steam/steam/steamapps/common/ELDEN\ RING/Game/eldenring.exe`.

For Dark Souls III:

```
$ cargo run --bin binary-mapper -- ds3 --exe "<game exe path>"
```

You can also set environment variables for the executable paths rather than passing them by flag every time. These take the form `MAPPER_{GAME}_{REGION}_EXE`. For example, instead of `er --ww-exe`, you can set `MAPPER_ER_WW_EXE`. Because Dark Souls III has the same mappings for all regions, it just takes `MAPPER_DS3_EXE`.

## Manual Mapping and Debugging

You can also print individual files directly to standard output. This can be useful for debugging, or for generating custom RVAs for one particular mod. To do this, use the `map` command:

```
$ cargo run --bin binary-mapper -- map --profile crates/util/mapper-profile.toml --exe <game exe path> --output rust > path/to/src/rva/rvas.rs
```

There are three different `--output` options:

* `rust` emits the contents of a Rust file that instantiates an `RvaBundle` struct, passing the given RVAs as initializers.
* `rust-struct` emits the definition of the `RvaBundle` struct. You can omit the `--exe` parameter for this output, since it doesn't actually locate the RVAs themselves.
* `print` prints the results in debug format, which can be useful when verifying that you've found the right RVA.

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
