use std::iter::Iterator;

use proc_macro::TokenStream;
use quote::*;
use syn::spanned::Spanned;
use syn::*;

/// A helper for [derive_researching] that returns a [syn::Result].
pub fn researching_helper(input: TokenStream) -> Result<TokenStream> {
    let struct_: ItemStruct = syn::parse(input)?;
    let name = &struct_.ident;
    let (impl_generics, ty_generics, where_clause) = struct_.generics.split_for_impl();

    let Fields::Named(fields) = struct_.fields else {
        return Err(Error::new(struct_.fields.span(), "expected named fields"));
    };
    let entries = fields
        .named
        .into_iter()
        .filter_map(|f| {
            if let Some(name) = f.ident.as_ref().map(|n| n.to_string())
                && (name.starts_with("unk") || name.starts_with("_unk"))
            {
                Some((name, f.span()))
            } else {
                None
            }
        })
        .map(|(n, s)| {
            let name = LitStr::new(&n, s);
            let ident = Ident::new(&n, s);
            quote_spanned! { s => (#name, &self.#ident) }
        });

    Ok(TokenStream::from(quote! {
        impl #impl_generics ::fromsoftware_shared::Researching for #name #ty_generics
            #where_clause
        {
            fn unknown_fields(&self) -> Vec<(&str, &dyn ::std::fmt::Debug)> {
                vec![
                    #(#entries),*
                ]
            }
        }
    }))
}
