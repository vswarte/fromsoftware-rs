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

/// A helper for [multi_param] that returns a [syn::Result].
pub fn multi_param_helper(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut input_trait: ItemTrait = syn::parse(input)?;
    let structs = Punctuated::<TypePath, Token![,]>::parse_terminated.parse(args)?;
    let fields = extract_fields(&mut input_trait)?;

    for field in &fields {
        let ident = &field.ident;
        let set_ident = format_ident!("set_{}", field.ident);
        let ty = &field.ty;
        input_trait.items.push(syn::parse2(quote_spanned! {
            field.span => fn #ident(&self) -> #ty;
        })?);
        input_trait.items.push(syn::parse2(quote_spanned! {
            field.span => fn #set_ident(&mut self, value: #ty);
        })?);
    }

    let impls = structs
        .into_iter()
        .map(|struct_| generate_impl(&input_trait.ident, struct_, &fields))
        .collect::<Result<Vec<_>>>()?;

    Ok(TokenStream::from(quote! {
        #input_trait

        #(#impls)*
    }))
}

struct MultiParamField {
    ident: Ident,
    ty: Type,
    span: Span,
}

/// Returns all fields in [trait_] declared with [multi_param_fields].
fn extract_fields(trait_: &mut ItemTrait) -> Result<Vec<MultiParamField>> {
    trait_
        .items
        .extract_if(.., |item| match item {
            TraitItem::Macro(mac) => mac.mac.path.is_ident("fields"),
            _ => false,
        })
        .filter_map(|item| match item {
            TraitItem::Macro(mac) => Some(mac),
            _ => None,
        })
        .flat_map(|mac| {
            let parser = |input: &ParseBuffer<'_>| {
                Punctuated::<Field, Token![,]>::parse_terminated_with(input, Field::parse_named)
            };
            let fields = match parser.parse2(mac.mac.tokens) {
                Ok(fields) => fields,
                Err(err) => return vec![Err(err)],
            };

            fields
                .into_iter()
                .map(|field| {
                    if !field.attrs.is_empty() {
                        Err(Error::new(
                            field.attrs[0].span(),
                            "multi_param fields may not have attributes",
                        ))
                    } else if field.vis != Visibility::Inherited {
                        Err(Error::new(
                            field.span(),
                            "multi_param fields may not have attributes",
                        ))
                    } else {
                        let span = field.span();
                        Ok(MultiParamField {
                            ident: field.ident.unwrap(),
                            ty: field.ty,
                            span,
                        })
                    }
                })
                .collect()
        })
        .collect()
}

/// Generates an implementation of [trait_] for [target] which forwards getters
/// and setters for all fields in [fields] to methods of the same name.
fn generate_impl<'a>(
    trait_: &Ident,
    target: TypePath,
    fields: impl IntoIterator<Item = &'a MultiParamField>,
) -> Result<ItemImpl> {
    let mut result: ItemImpl = syn::parse2(quote! {
        impl #trait_ for #target {}
    })?;

    for MultiParamField { ident, ty, span } in fields {
        result.items.push(syn::parse2(quote_spanned! { *span =>
            fn #ident(&self) -> #ty {
                #target::#ident(self)
            }
        })?);

        let set_ident = format_ident!("set_{}", ident);
        result.items.push(syn::parse2(quote_spanned! { *span =>
            fn #set_ident(&mut self, value: #ty) {
                #target::#set_ident(self, value)
            }
        })?);
    }

    Ok(result)
}
