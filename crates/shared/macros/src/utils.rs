use proc_macro2::Span;
use std::iter::Iterator;

use quote::*;
use syn::*;

/// Constructs a new lifetime with the given name.
pub fn new_lifetime(name: &str, span: Span) -> Lifetime {
    Lifetime {
        apostrophe: span,
        ident: Ident::new(name, span),
    }
}

/// Returns a copy of [generics] with a copy of [lifetime] added to the
/// beginning.
pub fn add_lifetime(lifetime: &Lifetime, generics: &Generics) -> Generics {
    let Generics {
        lt_token,
        params,
        gt_token,
        where_clause,
    } = generics;

    if params.is_empty() {
        syn::parse2(quote! {< #lifetime >}).unwrap()
    } else {
        let params = params.into_iter();
        let item: ItemImpl = syn::parse2(quote! {
            impl #lt_token #lifetime, #(#params),* #gt_token Irrelevant #where_clause {}
        })
        .unwrap();
        item.generics
    }
}

/// Returns a copy of [generics] with a copy of [type_param] added to the end.
pub fn add_type_param(type_param: &TypeParam, generics: &Generics) -> Generics {
    let Generics {
        lt_token,
        params,
        gt_token,
        where_clause,
    } = generics;

    if params.is_empty() {
        syn::parse2(quote! {< #type_param >}).unwrap()
    } else {
        let params = params.into_iter();
        let item: ItemImpl = syn::parse2(quote! {
            impl #lt_token #(#params),*, #type_param #gt_token Irrelevant #where_clause {}
        })
        .unwrap();
        item.generics
    }
}
