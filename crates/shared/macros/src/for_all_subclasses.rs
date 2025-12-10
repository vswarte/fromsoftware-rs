use std::iter::Iterator;

use proc_macro::TokenStream;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::*;

use crate::utils::*;

/// A helper for [for_all_subclasses] that returns a [syn::Result].
pub fn for_all_subclasses_helper(input: TokenStream) -> Result<TokenStream> {
    let ImplWithVisibility(vis, impl_) = syn::parse(input)?;
    let ext_trait = get_ext_name(&impl_)?;
    let superclass = get_superclass_type(&impl_.self_ty)?;
    let mut items = impl_.items;
    let trait_items = extract_trait_items(&mut items)?;

    let type_param =
        syn::parse2(quote! { __Subclass: ::fromsoftware_shared::Subclass<#superclass> })?;
    let generics = add_type_param(&type_param, &impl_.generics);
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    Ok(TokenStream::from(quote! {
        /// An extension trait for all subclasses of [#superclass]. This
        /// makes it possible to use these methods even in generic
        /// contexts.
        #vis trait #ext_trait: Subclass<#superclass> {
            #(#trait_items)*
        }

        impl #impl_generics #ext_trait for __Subclass #where_clause {
            #(#items)*
        }
    }))
}

/// A normal [ItemImpl] with the sole exception that it may be preceded by a
/// visibility indicator.
struct ImplWithVisibility(Visibility, ItemImpl);

impl Parse for ImplWithVisibility {
    fn parse(input: ParseStream<'_>) -> Result<ImplWithVisibility> {
        Ok(ImplWithVisibility(input.parse()?, input.parse()?))
    }
}

/// Verifies that [type_] is of the form `Subclass<TypePath>` and returns the
/// inner type (which is to say the type of the superclass).
fn get_superclass_type(type_: &Type) -> Result<&TypePath> {
    if let Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    }) = type_
        && segments.len() == 1
        && let Some(PathSegment {
            ident: subclass,
            arguments:
                PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    args,
                    ..
                }),
        }) = segments.first()
        && subclass == "Subclass"
        && args.len() == 1
    {
        if let Some(GenericArgument::Type(Type::Path(path))) = args.first() {
            Ok(path)
        } else {
            Err(Error::new(args.span(), "expected superclass type"))
        }
    } else {
        Err(Error::new(type_.span(), "expected Subclass<...>"))
    }
}

/// Asserts that [impl_] has a single identifier in its trait positiona and
/// returns a clone of it.
fn get_ext_name(impl_: &ItemImpl) -> Result<Ident> {
    let Some((_, path, _)) = &impl_.trait_ else {
        return Err(Error::new(impl_.span(), "expected a trait name"));
    };

    if let Some(colon) = path.leading_colon {
        return Err(Error::new(colon.span(), "expected a bare identifier"));
    } else if path.segments.len() != 1 {
        return Err(Error::new(path.span(), "expected a bare identifier"));
    }

    let segment = path.segments.first().unwrap();
    if segment.arguments == PathArguments::None {
        Ok(segment.ident.clone())
    } else {
        Err(Error::new(segment.span(), "expected a bare identifier"))
    }
}

/// Extracts and returns all [items] that can be defined in a trait (constants,
/// functions, and sort of types).
///
/// Since types can't have defaults assigned in the trait itself, they're left
/// in [items] and a copy of the type name is added to the result.
fn extract_trait_items(items: &mut Vec<ImplItem>) -> Result<Vec<TraitItem>> {
    let mut result = items
        .extract_if(.., |item| {
            // TODO: Supports types as well once type defaults are no longer unstable
            matches!(item, ImplItem::Const(_) | ImplItem::Fn(_))
        })
        .map(|item| {
            Ok(match item {
                ImplItem::Const(const_) => {
                    assert_private(const_.vis)?;
                    TraitItem::Const(TraitItemConst {
                        attrs: const_.attrs,
                        const_token: const_.const_token,
                        ident: const_.ident,
                        generics: const_.generics,
                        colon_token: const_.colon_token,
                        default: Some((const_.eq_token, const_.expr)),
                        ty: const_.ty,
                        semi_token: const_.semi_token,
                    })
                }
                ImplItem::Fn(fn_) => {
                    assert_private(fn_.vis)?;
                    TraitItem::Fn(TraitItemFn {
                        attrs: fn_.attrs,
                        sig: fn_.sig,
                        default: Some(fn_.block),
                        semi_token: None,
                    })
                }
                _ => unreachable!(),
            })
        })
        .collect::<Result<Vec<_>>>()?;

    result.extend(items.iter().filter_map(|item| {
        let ImplItem::Type(type_) = item else {
            return None;
        };

        Some(TraitItem::Type(TraitItemType {
            attrs: type_.attrs.clone(),
            type_token: type_.type_token,
            ident: type_.ident.clone(),
            generics: type_.generics.clone(),
            colon_token: None,
            bounds: Default::default(),
            default: None,
            semi_token: type_.semi_token,
        }))
    }));

    Ok(result)
}

/// Returns a [Result::Error] if [vis] is anything other than
/// [Visibility::Inherited].
fn assert_private(vis: Visibility) -> Result<()> {
    if vis == Visibility::Inherited {
        Ok(())
    } else {
        Err(Error::new(
            vis.span(),
            "trait items must not have explicit visibility",
        ))
    }
}
