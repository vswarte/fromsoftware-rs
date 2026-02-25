use proc_macro::TokenStream;
use quote::*;
use syn::*;

mod multi_param;

mod for_all_subclasses;
mod stepper;
mod subclass;
mod superclass;
mod utils;

/// Annotates a struct as a Dantelion2 singleton to be looked up using a single
/// string argument.
///
/// This is only guaranteed to make the struct work with the
/// `fromsoftware_shared::singleton::get_instance` function. Any other added
/// functionality is considered an implementation detail and shouldn't be relied
/// upon.
#[proc_macro_attribute]
pub fn singleton(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct: ItemStruct = parse_macro_input!(input as ItemStruct);
    let input_struct_ident = input_struct.ident.clone();
    let dlrf_name = parse_macro_input!(args as LitStr).value();

    TokenStream::from(quote! {
        #input_struct

        impl ::fromsoftware_shared::FromSingleton for #input_struct_ident {
            fn name() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#dlrf_name)
            }
        }
    })
}

/// Annotates a trait to automatically generate getters and setters that forward
/// to methods of the same name in various structs.
///
/// This is used to create traits that encapsulate state that's shared across
/// multiple parameter definitions.
///
/// This trait takes as arguments the names of various structs for which it
/// should automatically generate an implementation. It should annotate a trait
/// that contains a `fields!` macro, using the same named field syntax that a
/// struct uses. For each field, a getter and setter is generated both in the
/// trait and in its implementation for each struct.
///
/// ```rs
/// #[multi_param(EQUIP_PARAM_ACCESSORY_ST, EQUIP_PARAM_GOODS_ST)]
/// pub trait EquipParamPassive: EquipParam {
///     fields! {
///         sfx_variation_id: i32,
///         ref_category: u8,
///         sp_effect_category: u8,
///         shop_lv: i16,
///     }
/// }
/// ```
///
/// ## Field Attributes
///
/// This can also be used as to annotate field attributes.
///
/// ### `rename()`
///
/// You can use the `rename()` annotation to rename the field this targets for a
/// particular struct. This can be useful if the same logical field has
/// different names, which sometimes happens due to typos in FromSoftware's
/// internal names.
///
/// The syntax is `#[multi_param(rename(param = ..., name = ...))]`. The struct
/// must exactly match one of the structs passed to the outer annotation, and
/// the `name` must be a valid identifier.
///
/// ```rs
/// #[multi_param(
///     EQUIP_PARAM_ACCESSORY_ST,
///     EQUIP_PARAM_GOODS_ST,
///     EQUIP_PARAM_PROTECTOR_ST,
///     EQUIP_PARAM_WEAPON_ST
/// )]
/// pub trait EquipParam {
///     fields! {
///         weight: f32,
///         basic_price: i32,
///         sell_value: i32,
///         sort_id: i32,
///         vagrant_item_lot_id: i32,
///         #[multi_param(
///             rename(param = EQUIP_PARAM_PROTECTOR_ST, name = "vagrant_bonusene_drop_item_lot_id"),
///             rename(param = EQUIP_PARAM_WEAPON_ST, name = "vagrant_bonusene_drop_item_lot_id"),
///         )]
///         vagrant_bonus_ene_drop_item_lot_id: i32;
///         vagrant_item_ene_drop_item_lot_id: i32,
///     }
/// }
/// ```rs
///
/// ## Enums
///
/// Each trait also generates two enums, one mutable and one immutable, which
/// can be used in contexts where it's relevant *which* param type in particular
/// you have. These enums have a variant for each defined parameter type. The
/// above example would define:
///
/// ```rs
/// pub enum EquipParamStruct<'a> {
///   EQUIP_PARAM_ACCESSORY_ST(&'a EQUIP_PARAM_ACCESSORY_ST),
///   EQUIP_PARAM_GOODS_ST(&'a EQUIP_PARAM_GOODS_ST),
///   EQUIP_PARAM_PROTECTOR_ST(&'a EQUIP_PARAM_PROTECTOR_ST),
///   EQUIP_PARAM_WEAPON_ST(&'a EQUIP_PARAM_WEAPON_ST),
/// }
///
/// impl EquipParamStruct<'_> {
///   pub fn as_dyn(&self) -> &dyn EquipParam;
/// }
///
/// pub enum EquipParamStructMut<'a> {
///   EQUIP_PARAM_ACCESSORY_ST(&'a mut EQUIP_PARAM_ACCESSORY_ST),
///   EQUIP_PARAM_GOODS_ST(&'a mut EQUIP_PARAM_GOODS_ST),
///   EQUIP_PARAM_PROTECTOR_ST(&'a mut EQUIP_PARAM_PROTECTOR_ST),
///   EQUIP_PARAM_WEAPON_ST(&'a mut EQUIP_PARAM_WEAPON_ST),
/// }
///
/// impl EquipParamStructMut<'_> {
///   pub fn as_dyn(&mut self) -> &mut dyn EquipParam;
/// }
/// ```
///
/// You can access these using the `as_enum()` and `as_enum_mut()` methods.
#[proc_macro_attribute]
pub fn multi_param(args: TokenStream, input: TokenStream) -> TokenStream {
    match multi_param::multi_param_helper(args, input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}

/// A derive macro for `fromsoftware_shared::Subclass`.
///
/// ## Finding the RVA
///
/// This adds an implementation of `Subclass` that gets its VMT address from a
/// standard RVA struct. This assumes:
///
/// * The crate using this contains a `crate::rva` module that exposes a `get()`
///   function.
///
/// * The `get()` function's return value has a public field whose name is a
///   snake-case version of this struct's name, followed by `_vmt`.
///
/// For example, `ChrIns` uses `crate::rva::get().chr_ins_vmt` as its VMT RVA.
///
/// ## Determining the Superclass
///
/// By default, the type of the first field in the subclass is used as the
/// superclass. You can explicitly choose one or more superclasses instead using
/// the `#[subclass(base = SuperclassType)]` attribute on the struct.
///
/// ## Additional Features
///
/// This macro will also add trait implementations for `AsRef<SuperclassType>`,
/// `AsMut<SuperclassType>`, and `TryFrom<&SuperclassType>`.
///
/// It will also implement `Deref<Target = SuperclassType>` and `DerefMut`, but
/// because a type can only have one implementation of these traits, if this
/// declares multiple superclasses they will only be implemented for the first
/// one. Since types can be transitively dereferenced, be sure to order the
/// bottommost superclass first so that all superclass methods can be accessed.
///
/// ## Safety
///
/// The `fromsoftware_shared::Subclass` trait is unsafe, and even though there's
/// currently no way to require that a derive macro be explicitly flagged as
/// unsafe, this does not add any additional safety guarantees beyond a manual
/// implementation. PLease read the `Subclass` documentation carefully to
/// understand the requirements to use this safety.
#[proc_macro_derive(Subclass, attributes(subclass))]
pub fn derive_subclass(input: TokenStream) -> TokenStream {
    match subclass::subclass_helper(input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}

/// A derive macro for `fromsoftware_shared::Superclass`.
///
/// ## Finding the RVA
///
/// This adds an implementation of `Subclass` that gets its VMT address from a
/// standard RVA struct. This assumes:
///
/// * The crate using this contains a `crate::rva` module that exposes a `get()`
///   function.
///
/// * The `get()` function's return value has a public field whose name is a
///   snake-case version of this struct's name, followed by `_vmt`.
///
/// For example, `ChrIns` uses `crate::rva::get().chr_ins_vmt` as its VMT RVA.
///
/// ## Subclass Enums
///
/// By default, this macro will just generate a straightforward implementation
/// of `Superclass`. But if you want, you can add a
/// `#[superclass(children(ChildName1, ChildName2))]` attribute to the struct.
/// If you do, the macro will also define two enums, one immutable and one
/// mutable.
///
/// For example:
///
/// ```rs
/// #[repr(C)]
/// #[derive(Superclass)]
/// #[superclass(children(Cow, Pig))]
/// pub struct Animal {
///   _vftable: usize,
/// }
/// ```
///
/// will generate
///
/// ```rs
/// pub enum AnimalSubclasses<'sub> {
///   Cow(&'sub Cow),
///   Pig(&'sub Pig),
///   Animal(&'sub Animal),
/// }
///
/// pub enum AnimalSubclassesMut<'sub> {
///   Cow(&'sub mut Cow),
///   Pig(&'sub mut Pig),
///   Animal(&'sub mut Animal),
/// }
///
/// impl AnimalSubclasses<'_> {
///   pub fn superclass(&self) -> &Animal;
/// }
///
/// impl AnimalSubclassesMut<'_> {
///   pub fn superclass(&self) -> &Animal;
///   pub fn superclass_mut(&mut self) -> &mut Animal;
/// }
///
/// impl<'sub> From<AnimalSubclassesMut<'sub>> for AnimalSubclasses<'sub> {}
/// impl<'sub> From<&'sub T> for AnimalSubclasses<'sub> where T: Subclass<Animal> {}
/// impl<'sub> From<&'sub mut T> for AnimalSubclassesMut<'sub> where T: Subclass<Animal> {}
/// ```
///
/// ## Safety
///
/// The `fromsoftware_shared::Superclass` trait is unsafe, and even though
/// there's currently no way to require that a derive macro be explicitly
/// flagged as unsafe, this does not add any additional safety guarantees beyond
/// a manual implementation. PLease read the `Superclass` documentation
/// carefully to understand the requirements to use this safety.
#[proc_macro_derive(Superclass, attributes(superclass))]
pub fn derive_superclass(input: TokenStream) -> TokenStream {
    match superclass::superclass_helper(input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}

/// A proc macro attribute for defining an extension trait that makes a set of
/// methods available for all subclasses of a superclass.
///
/// This expects to be used on a trait impl whose trait name is **not defined**
/// and whose target is `Subclass<...>`. This impl should include functions.
/// Unlike normal trait implementations, this impl **should have a visibility
/// modifier** (unless you want it to be private). For example:
///
/// ```rs
/// #[for_all_subclasses]
/// pub impl ChrInsExt for Subclass<ChrIns> {
///     fn apply_speffect(&mut self, sp_effect: i32, dont_sync: bool) {
///         let rva = Program::current()
///             .rva_to_va(rva::get().chr_ins_apply_speffect)
///             .unwrap();
///
///         let call = unsafe { transmute::<u64, extern "C" fn(&mut ChrIns, i32, bool) -> u64>(rva) };
///         call(self, sp_effect, dont_sync);
///     }
///
///     fn remove_speffect(&mut self, sp_effect: i32) {
///         let rva = Program::current()
///             .rva_to_va(rva::get().chr_ins_remove_speffect)
///             .unwrap();
///
///         let call = unsafe { transmute::<u64, extern "C" fn(&mut ChrIns, i32) -> u64>(rva) };
///         call(self, sp_effect);
///     }
/// }
/// ```
///
/// This will define a trait with the given name and visibility, then implement
/// it for all subclasses of the given superclass. This allow superclass methods
/// to be freely called on any subclass, delegating to the superclass they
/// contain.
#[proc_macro_attribute]
pub fn for_all_subclasses(_args: TokenStream, input: TokenStream) -> TokenStream {
    match for_all_subclasses::for_all_subclasses_helper(input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}

/// A derive macro that implements the StepperStates trait on a given enum.
///
/// - The enum must be exhaustive (represent all states and no more).
/// - The enum must have a -1 state for inactive steppers.
/// - The enum must have no gaps in the discriminants.
///
/// # Safety
///
/// The implementer must ensure that the enum is exhaustive as unknown discriminants can be used to
/// trigger undefined behavior.
/// The implementer must ensure that the enum does not have more states than the game defines.
/// Failing to do so will allow for out-of-bound access to the stepper array.
/// The implementer must ensure that the enum discriminants have no gaps. Failing to do so will
/// allow out of bounds access to the stepper array as well as cause unknown discriminants.
#[proc_macro_derive(StepperStates)]
pub fn derive_stepper_states(input: TokenStream) -> TokenStream {
    match stepper::stepper_states_helper(input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}
