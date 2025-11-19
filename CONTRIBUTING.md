# Contributing to fromsoftware-rs

This document serves to make contributing a bit more straight forward. It does this by explaining
some of the projects internal structure, some principles and conventions around documenting structures
that ordinarily reside in foreign memory as well as some naming/documentation guidelines.

## Projects goals

Fromsoftware-rs aims to be a sane bindings library for various FromSoftware games.
The projects goal is to thoroughly describe the games binaries such that the community can interact with the engine as well as gameplay aspects while avoiding the tedium of having to reverse engineer the binaries for themselves.

## Submitting pull requests

When submitting pull requests you are expected to fix any clippy lints, code style lints and any feedback from the projects maintainers.
Review times will vary especially when you are PRing new game structures since these need to be verified.
To make review easier in such situations please include a list of relevant RVAs and tell us what game version the RVA is for.

## Documenting game structures

### Struct layouts

Always annotate game structs with `#[repr(C)]` to force C-style struct layouts. Padding should be implicit.

Bad:
```rust
// WRONG: lacking a `#[repr(C)]`. Might sometimes work but the layout can sporadically change between rust vers.
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
Exposing a field before properly understood will potentially break consumer code when we want to change the field when learning more about its nature.

### Documenting fields

When documenting a field, please note in a rust doc comment `///` how the game uses this field.
A field with a proper name but no further explanation is often just as useful as an unk field.
Please do not put game version specific data in the comments, for example an RVA to relevant function will most likely be useless when the game updates.

Bad:
```rust
    // WRONG: comment is practically useless.
    /// Area ID of multiplay start.
    /// Used by 0x1412312312.
    pub multiplay_start_area_id: u32, 
```

Good:
```rust
    /// Sent by host to clients on connect in packet 90 (0x5A).
    /// Contains the ID of the play area the host was in when the client connected.
    /// If current multiplay area ID has different boss ID than this one, player will be warped to latest stored position.
    /// Setting this to 0 will disable this.
    pub multiplay_start_area_id: u32,
```

### Pointers

#### Pointers and OwnedPtr

Pointers in structures are usually expressed with either `NonNull` or `OwnedPtr` depending on if its safe to acquire a mutable reference.

`NonNull` is the most conservative of the two and should be the used if you're unsure about the conditions for `OwnedPtr` being applicable.

`OwnedPtr` is a pointer type that indicates that the structure containing it owns the memory backing the pointed-to structure.
Determining if your pointer should be an `OwnedPtr` can be hard, but there are a few things you can look out for:
1. the game itself only mutates the structure that is pointed using this pointer.
2. the pointee is constructed and destructing as part of the structure holding the pointer. This implies that the lifecycle of the two objects are the same.

`OwnedPtr` implements `Sync` and `Send`. Because of this you should never use `OwnedPtr` with pointers that leads to or can lead to stack memory.
`OwnedPtr` allows for a single mutable borrow and multiple immutable ones. Because of this you should never have two `OwnedPtr`s referencing to the same memory as this would create the possibility for undefined behavior.
It is fine to have one `OwnedPtr` and multiple `NonNull`s pointing to the same memory since getting any kind of reference through the `NonNull` will be explicitly unsafe.

#### Nullable pointers

Null pointers can be represented by wrapping a pointer type in `Option`.
For example `Option<OwnedPtr>` and `Option<NonNull>` are guaranteed to use an 8-byte `0x0` for its `None` state.

### Virtual method tables

VMTs are described using tremwil's `vtable-rs`. VMTs mappings are much preferred over fixed RVAs where possible since they're less likely to break between updates.

If the methods code mutates the class in any capacity it should take in a mutable reference to self on the method signature.

Unfortunately it is currently not possible to omit unknown VMT entries with `vtable-rs`. 
If it's not known what a particular VMT entry does we use the naming scheme `unk_28` where `28` represents the hex offset of the vmt slot itself.
Do not use the VMT slot index, with `unk_7` being an example for a method in the 8th slot.

Prefer using references over raw pointers for parameters. 
When you type a parameter as a raw pointer it's safe to pass down a reference, however typing a parameter to a raw pointer also allows for null pointers to be passed in.
Using references should guarantee a non-null pointer as well as the pointed-at memory being initialized.

Bad:
```rust
#[vtable_rs::vtable]
pub trait CSBulletStateVmt {
    fn destructor(&mut self, should_free: bool);

    // WRONG: Allow mutations against an immutable borrow.
    fn set_bullet_param_and_get_hit_bullet(&self, row_id: u32) -> i32;

    // WRONG: Using raw pointer for CSBulletIns instead of a reference.
    fn on_update(&mut self, bullet: *mut CSBulletIns, dt: f32);

    fn on_creation(&mut self, bullet: &mut CSBulletIns);

    fn on_death(&mut self, bullet: &mut CSBulletIns);

    // WRONG: Using the VMT slot # instead of the hex offset for the unk method.
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

The signature for unknown methods in VMTs may change without warning as users should not be using them in the first place.

### Tests

For structures with an explicit alloc or free call its good to have a `size_of` assert to ensure the size of the struct doesn't change when filling in more fields later down the line.
This prevents fields past the changed area to suddenly be at wrong offsets. 
For classes with a VMT you can usually find a free call in the destructor at VMT slot 0 or 1.

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
