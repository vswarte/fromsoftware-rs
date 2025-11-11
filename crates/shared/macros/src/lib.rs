use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitStr};

/// Annotates a struct as a Dantelion2 singleton to be looked up using a single
/// string argument.
///
/// This is only guaranteed to make the struct work with the
/// `fromsoftware_shared::singleton::get_instance` function. Any other added
/// functionality is considered an implementation detail and shouldn't be relied
/// upon.
#[proc_macro_attribute]
pub fn singleton(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct: ItemStruct = syn::parse_macro_input!(input as ItemStruct);
    let input_struct_ident = input_struct.ident.clone();
    let dlrf_name = syn::parse_macro_input!(args as LitStr).value();

    TokenStream::from(quote! {
        #input_struct

        impl ::shared::FromSingleton for #input_struct_ident {
            fn name() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#dlrf_name)
            }
        }
    })
}
