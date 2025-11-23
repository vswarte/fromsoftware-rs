# Contributing to fromsoftware-rs

This document serves to make contributing a bit more straightforward. It does this by explaining
some of the project's internal structure, some principles and conventions around documenting structures
that ordinarily reside in foreign memory, as well as some naming/documentation guidelines.

## Projects goals

Fromsoftware-rs aims to be a sane bindings library for various FromSoftware games.
The project's goal is to thoroughly describe the game's binaries such that the community can interact with the engine as well as gameplay aspects while avoiding the tedium of having to reverse-engineer the binaries for themselves.

## Submitting pull requests

When submitting pull requests you are expected to fix any clippy lints, code style lints, and any feedback from the projects maintainers.
Review times will vary especially when you are PRing new game structures since these need to be verified.
To make review easier in these situations, please include a list of relevant RVAs and tell us what game version the RVA is for.

## Documenting game structures

### Struct layouts

Always annotate game structs with `#[repr(C)]` to force C-style struct layouts. Padding should be implicit.

Bad:
```rust
// WRONG: lacking a `#[repr(C)]`. Might sometimes work but the layout can
// sporadically change between Rust versions.
#[shared::singleton("CSWorldSceneDrawParamManager")]
pub struct CSWorldSceneDrawParamManager {
    vftable: usize,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    world_block_info_count: u32,
    // WRONG: padding is explicit. Omitting it should result in the same layout.
    _pad14: u32,
    unk18: u64,
    unk20: u64,
    pub world_area_blocks: DoublyLinkedList<CSWorldAreaBlockSceneDrawParam>,
    pub scene_draw_param: CSFD4SceneDrawParam,
}
```

Good:
```rust
#[repr(C)]
#[shared::singleton("CSWorldSceneDrawParamManager")]
pub struct CSWorldSceneDrawParamManager {
    vftable: usize,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    world_block_info_count: u32,
    unk18: u64,
    unk20: u64,
    pub world_area_blocks: DoublyLinkedList<CSWorldAreaBlockSceneDrawParam>,
    pub scene_draw_param: CSFD4SceneDrawParam,
}
```

### Exposing fields

We generally do not want to make fields public if we do not fully understand them or they are expected to change in name or type.
Exposing a field before it's properly understood will potentially break consumer code when we want to change the field as we learn more about its nature.

### Documenting fields

When documenting a field, please note in a Rust doc comment (`///`) how the game uses this field.
A field with a name but no further explanation is often no more useful than an unk field.
Please do not put game-version-specific information in the comments; for example, an RVA to a relevant function will most likely be useless when the game updates.

Bad:
```rust
    // WRONG: This comment is practically useless.
    /// Area ID of multiplay start.
    /// Used by 0x1412312312.
    pub multiplay_start_area_id: u32, 
```

Good:
```rust
    /// Sent by the host to clients on connect in packet 90 (0x5A). Contains the
    /// ID of the play area the host was in when the client connected. If the
    /// current multiplay area ID has different boss ID than this one, the
    /// player will be warped to latest stored position. Setting this to 0 will
    /// disable this behavior.
    pub multiplay_start_area_id: u32,
```

### Pointers

#### Pointers and OwnedPtr

Pointers in structures are usually expressed with either `NonNull` or `OwnedPtr` depending on whether it's safe for calling code to acquire a mutable reference.

`NonNull` is the most conservative of the two and should be the used if you're unsure whether it meets the conditions for `OwnedPtr`.

`OwnedPtr` is a custom pointer type that indicates that the structure containing it owns the memory backing the referenced structure.
Determining if your pointer should be an `OwnedPtr` can be hard, but there are a few things you can look for:
1. The game itself only mutates the referenced structure through this pointer.
2. The referenced structure is constructed and destructed as part of the parent structure's constructor and destructor. This implies that the lifecycles of the two objects are the same.

`OwnedPtr` implements `Sync` and `Send`. Because of this you should never use `OwnedPtr` with pointers that can lead to stack memory.
`OwnedPtr` also allows for a single mutable borrow and multiple immutable ones. Because of this, you should never have two `OwnedPtr`s referencing to the same memory, since this would create the possibility for undefined behavior.
It's fine to have one `OwnedPtr` and multiple `NonNull`s pointing to the same memory since getting any kind of reference through the `NonNull` is explicitly unsafe.

#### Nullable pointers

Null pointers can be represented by wrapping a pointer type in `Option`, as in `Option<OwnedPtr>` and `Option<NonNull>`.
These are guaranteed to use an 8-byte `0x0` for `None`, so they're compatible with the C++ null representation used by the games.

### Virtual method tables

VMTs are described using tremwil's [`vtable-rs`]. VMT mappings are strongly preferred over fixed RVAs where possible since they're less likely to break between updates.

[`vtable-rs`]: https://crates.io/crates/vtable-rs

If the C++ method mutates the class in any way, the Rust signature should use `&mut self`.

Unfortunately it is not currently possible to omit unknown VMT entries with `vtable-rs`. 
If we don't know what a particular VMT entry does, we use the naming scheme `unk_28` where `28` represents the hex offset of the vmt slot itself.
**Do not** use the VMT slot index—the number should always be a multiple of 8.

Prefer using references over raw pointers for parameters.
When use a raw pointer type for a parameter, it's safe to pass in a reference, but it also allows null pointers to be passed in.
Using references guarantees both a non-null pointer and initialized memory.

Bad:
```rust
#[vtable_rs::vtable]
pub trait CSBulletStateVmt {
    fn destructor(&mut self, should_free: bool);

    // WRONG: Allows mutations against an immutable borrow.
    fn set_bullet_param_and_get_hit_bullet(&self, row_id: u32) -> i32;

    // WRONG: Uses a raw pointer for CSBulletIns instead of a reference.
    fn on_update(&mut self, bullet: *mut CSBulletIns, dt: f32);

    fn on_creation(&mut self, bullet: &mut CSBulletIns);

    fn on_death(&mut self, bullet: &mut CSBulletIns);

    // WRONG: Uses the VMT slot number instead of the hex offset.
    fn unk5(&mut self, param_row: usize, param_3: usize);

    // ...
}
```

Good:
```rust
#[vtable_rs::vtable]
pub trait CSBulletStateVmt {
    fn destructor(&mut self, should_free: bool);

    fn set_bullet_param_and_get_hit_bullet(&mut self, row_id: u32) -> i32;

    fn on_update(&mut self, bullet: &mut CSBulletIns, dt: f32);

    fn on_creation(&mut self, bullet: &mut CSBulletIns);

    fn on_death(&mut self, bullet: &mut CSBulletIns);

    fn unk28(&mut self, param_row: usize, param_3: usize);

    // ...
}
```

We expect users not to use unknown methods in VMTs, so we reserve the right to change their signatures without warning.

### Tests

For structures with an explicit alloc or free call it's a good idea to have a `size_of` assert to ensure the size of the struct doesn't change when filling in more fields later down the line.
This helps ensure that fields past the changed area remain at the correct offsets.
For classes with a VMT, you can usually find a free call with the expected size in the destructor at VMT slot 0 or 1.

Ex:
```rust
#[repr(C)]
#[shared::singleton("CSFeMan")]
pub struct CSFeManImp {
    // ...
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x8420, size_of::<CSFeManImp>());
    }
}
```
