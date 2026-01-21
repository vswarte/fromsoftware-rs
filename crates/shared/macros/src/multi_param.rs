use std::collections::{HashMap, hash_map::Entry};
use std::iter::{IntoIterator, Iterator};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::*;
use syn::{
    meta::ParseNestedMeta,
    parse::{ParseBuffer, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
};

/// A helper for [multi_param] that returns a [syn::Result].
pub fn multi_param_helper(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut input_trait: ItemTrait = syn::parse(input)?;
    let structs = Punctuated::<TypePath, Token![,]>::parse_terminated
        .parse(args)?
        .into_iter()
        .collect::<Vec<_>>();
    let fields = extract_fields(&mut input_trait, &structs)?;

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

    let trait_name = &input_trait.ident;
    let enum_ = format_ident!("{}Struct", trait_name);
    let enum_mut = format_ident!("{}StructMut", trait_name);

    let impls = structs
        .iter()
        .map(|struct_| generate_impl(&trait_name, struct_, &fields))
        .collect::<Result<Vec<_>>>()?;

    input_trait.items.push(syn::parse2(quote! {
        /// Returns an [#enum_] representing the type of this parameter
        /// struct.
        fn as_enum(&self) -> #enum_<'_>;
    })?);

    input_trait.items.push(syn::parse2(quote! {
        /// Returns an [#enum_mut] representing the type of this parameter
        /// struct.
        fn as_enum_mut(&mut self) -> #enum_mut<'_>;
    })?);

    Ok(TokenStream::from(quote! {
        #input_trait

        #(#impls)*

        /// An enum of possible structs that [#trait_name] can be.
        pub enum #enum_<'a> {
            #(
                #[allow(non_camel_case_types)]
                #structs(&'a #structs)
            ),*
        }

        /// A mutable enum of possible structs that [#trait_name] can be.
        pub enum #enum_mut<'a> {
            #(
                #[allow(non_camel_case_types)]
                #structs(&'a mut #structs)
            ),*
        }
    }))
}

/// A field declared in the `fields!` macro that should be dispatched to each
/// parameter struct.
struct MultiParamField {
    /// The default field name.
    ident: Ident,

    /// The field's type.
    ty: Type,

    /// The span at which the field was declared.
    span: Span,

    /// Specialized field names to use for particular structs.
    renames: HashMap<TypePath, Ident>,
}

/// Returns all fields in [trait_] declared with [multi_param_fields].
fn extract_fields(trait_: &mut ItemTrait, structs: &[TypePath]) -> Result<Vec<MultiParamField>> {
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
                .map(|mut field| {
                    let attributes = extract_field_attributes(&mut field)?;
                    let mut renames = HashMap::new();
                    for attr in attributes {
                        match attr {
                            FieldAttribute::Rename(param, name) => {
                                if !structs.contains(&param) {
                                    return Err(Error::new(
                                        param.span(),
                                        "this isn't one of the multi_param() arguments",
                                    ));
                                }

                                match renames.entry(param) {
                                    Entry::Occupied(o) => {
                                        return Err(Error::new(o.key().span(), "duplicate param"));
                                    }
                                    Entry::Vacant(v) => v.insert(name),
                                }
                            }
                        };
                    }

                    if !field.attrs.is_empty() {
                        Err(Error::new(
                            field.attrs[0].span(),
                            "multi_param fields may only have #[multi_param(...)] attributes",
                        ))
                    } else if field.vis != Visibility::Inherited {
                        Err(Error::new(
                            field.span(),
                            "multi_param fields must have default visibility",
                        ))
                    } else {
                        let span = field.span();
                        Ok(MultiParamField {
                            ident: field.ident.unwrap(),
                            ty: field.ty,
                            span,
                            renames,
                        })
                    }
                })
                .collect()
        })
        .collect()
}

/// A `multi_param()` attribute on a field.
enum FieldAttribute {
    /// `rename(struct = ..., name = ...)`
    Rename(TypePath, Ident),
}

/// Removes all `#[multi_param(...)]` attributes from [field] and returns them
/// as [FieldAttribute]s.
fn extract_field_attributes(field: &mut Field) -> Result<Vec<FieldAttribute>> {
    field
        .attrs
        .extract_if(.., |attr| attr.path().is_ident("multi_param"))
        .flat_map(|attr| {
            let mut attributes = Vec::new();
            if let Err(err) = attr.parse_nested_meta(|meta| {
                attributes.push(Ok(parse_field_attribute(meta)?));
                Ok(())
            }) {
                return vec![Err(err)];
            }
            attributes
        })
        .collect()
}

/// Parses a single nested meta item inside a `#[multi_param(...)]` attribute on
/// a field in `fields!`.
fn parse_field_attribute(meta: ParseNestedMeta<'_>) -> Result<FieldAttribute> {
    if !meta.path.is_ident("rename") {
        return Err(meta.error("unrecognized attribute"));
    }

    let mut param: Option<TypePath> = None;
    let mut name: Option<Ident> = None;
    meta.parse_nested_meta(|arg| {
        if arg.path.is_ident("param") {
            param = Some(arg.value()?.parse()?);
            Ok(())
        } else if arg.path.is_ident("name") {
            name = Some(arg.value()?.parse::<LitStr>()?.parse()?);
            Ok(())
        } else {
            Err(arg.error("unrecognized argument"))
        }
    })?;

    match (param, name) {
        (Some(param), Some(name)) => Ok(FieldAttribute::Rename(param, name)),
        (None, _) => Err(meta.error("missing argument \"param\"")),
        (_, None) => Err(meta.error("missing argument \"name\"")),
    }
}

/// Generates an implementation of `trait_` for `target` which forwards getters
/// and setters for all fields in `fields` to methods of the same name.
///
/// Generates an `as_enum()` method that returns the given [enum_].
fn generate_impl<'a>(
    trait_: &Ident,
    target: &TypePath,
    fields: impl IntoIterator<Item = &'a MultiParamField>,
) -> Result<ItemImpl> {
    let enum_ = format_ident!("{}Struct", trait_);
    let enum_mut = format_ident!("{}StructMut", trait_);

    let mut result: ItemImpl = syn::parse2(quote! {
        impl #trait_ for #target {
            fn as_enum(&self) -> #enum_<'_> {
                #enum_::#target(self)
            }

            fn as_enum_mut(&mut self) -> #enum_mut<'_> {
                #enum_mut::#target(self)
            }
        }
    })?;

    for MultiParamField {
        ident,
        ty,
        span,
        renames,
    } in fields
    {
        let target_ident = renames.get(&target).unwrap_or(ident);

        result.items.push(syn::parse2(quote_spanned! { *span =>
            fn #ident(&self) -> #ty {
                #target::#target_ident(self)
            }
        })?);

        let set_ident = format_ident!("set_{}", ident);
        let set_target_ident = format_ident!("set_{}", target_ident);
        result.items.push(syn::parse2(quote_spanned! { *span =>
            fn #set_ident(&mut self, value: #ty) {
                #target::#set_target_ident(self, value)
            }
        })?);
    }

    Ok(result)
}
