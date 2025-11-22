use proc_macro::TokenStream;
use quote::*;
use syn::*;

mod multi_param;

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

        impl ::from_singleton::FromSingleton for #input_struct_ident {
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
#[proc_macro_attribute]
pub fn multi_param(args: TokenStream, input: TokenStream) -> TokenStream {
    match multi_param::multi_param_helper(args, input) {
        Ok(stream) => stream,
        Err(err) => err.into_compile_error().into(),
    }
}
