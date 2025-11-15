use std::iter::{IntoIterator, Iterator};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::*;
use syn::{
    parse::{ParseBuffer, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

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
