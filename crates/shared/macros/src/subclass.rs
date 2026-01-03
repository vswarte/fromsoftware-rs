use proc_macro2::Span;
use std::iter::Iterator;

use inflections::Inflect;
use proc_macro::TokenStream;
use quote::*;
use syn::spanned::Spanned;
use syn::*;

use crate::utils::*;

/// A helper for [derive_subclass] that returns a [syn::Result].
pub fn subclass_helper(input: TokenStream) -> Result<TokenStream> {
    let mut subclass_struct: ItemStruct = syn::parse(input)?;
    let subclass = subclass_struct.ident.clone();
    let superclasses = extract_superclasses(&mut subclass_struct)?;
    let first_superclass = superclasses.first().unwrap();
    let rva = Ident::new(
        &format!("{}_vmt", subclass.to_string().to_snake_case()),
        subclass.span(),
    );
    let subclass_string = LitStr::new(&subclass.to_string(), subclass.span());

    let (impl_generics, ty_generics, where_clause) = subclass_struct.generics.split_for_impl();
    let lifetime = new_lifetime("sub", Span::call_site());
    let lt_generics = add_lifetime(&lifetime, &subclass_struct.generics);
    let (lt_impl_generics, _, _) = lt_generics.split_for_impl();
    Ok(TokenStream::from(quote! {
        #(
            // TODO: Once Rust supports `unsafe(derive(...))`, make sure that's
            // required here.
            unsafe impl #impl_generics ::fromsoftware_shared::Subclass<#superclasses> for #subclass
                #ty_generics #where_clause
            {
                fn vmt_rva() -> ::pelite::pe64::Rva {
                    crate::rva::get().#rva
                }
            }

            impl #impl_generics AsRef<#superclasses> for #subclass #ty_generics #where_clause {
                fn as_ref(&self) -> &#superclasses {
                    self.superclass()
                }
            }

            impl #impl_generics AsMut<#superclasses> for #subclass #ty_generics #where_clause {
                fn as_mut(&mut self) -> &mut #superclasses {
                    self.superclass_mut()
                }
            }

            impl #lt_impl_generics TryFrom<&#lifetime #superclasses> for &#lifetime #subclass
                #ty_generics #where_clause
            {
                type Error = ::fromsoftware_shared::TryFromSuperclassError;

                fn try_from(value: &#lifetime #superclasses)
                            -> ::std::result::Result<Self, Self::Error> {
                    use ::fromsoftware_shared::Superclass;
                    value
                        .as_subclass()
                        .ok_or_else(|| {
                            Self::Error::new(#subclass_string.to_string())
                        })
                }
            }
        )*

        impl #impl_generics ::std::ops::Deref for #subclass #ty_generics #where_clause {
            type Target = #first_superclass;

            fn deref(&self) -> &Self::Target {
                self.superclass()
            }
        }

        impl #impl_generics ::std::ops::DerefMut for #subclass #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.superclass_mut()
            }
        }
    }))
}

/// Returns the types of the superclasses, determined by the `subclass(base =
/// ...)` attribute if it exists or else the type of the first field in the
/// struct.
fn extract_superclasses(subclass: &mut ItemStruct) -> Result<Vec<TypePath>> {
    let explicit = subclass
        .attrs
        .extract_if(.., |attr| attr.path().is_ident("subclass"))
        .flat_map(|attr| {
            let mut superclasses = Vec::new();
            if let Err(err) = attr.parse_nested_meta(|meta| {
                if !meta.path.is_ident("base") {
                    Err(meta.error("unrecognized attribute"))
                } else {
                    superclasses.push(Ok(meta.value()?.parse()?));
                    Ok(())
                }
            }) {
                vec![Err(err)]
            } else {
                superclasses
            }
        })
        .collect::<Result<Vec<TypePath>>>()?;
    if !explicit.is_empty() {
        return Ok(explicit);
    }

    let Fields::Named(ref named) = subclass.fields else {
        return Err(Error::new(
            subclass.span(),
            "subclass must be a C-style struct",
        ));
    };

    let Some(first) = named.named.first() else {
        return Err(Error::new(
            subclass.span(),
            "subclass must have at least one field",
        ));
    };

    let Type::Path(ref superclass) = first.ty else {
        return Err(Error::new(
            subclass.span(),
            "subclass's first field must be a struct type",
        ));
    };

    Ok(vec![superclass.clone()])
}
