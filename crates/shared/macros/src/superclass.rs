use proc_macro2::Span;
use std::iter::Iterator;

use inflections::Inflect;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::punctuated::Punctuated;
use syn::*;

use crate::utils::*;

/// A helper for [derive_superclass] that returns a [syn::Result].
pub fn superclass_helper(input: TokenStream) -> Result<TokenStream> {
    let mut superclass_struct: ItemStruct = syn::parse(input)?;
    let subclass_enums = build_subclass_enums(&mut superclass_struct)?;
    let superclass = &superclass_struct.ident;
    let rva = Ident::new(
        &format!("{}_vmt", superclass.to_string().to_snake_case()),
        superclass.span(),
    );

    let (impl_generics, ty_generics, where_clause) = superclass_struct.generics.split_for_impl();
    Ok(TokenStream::from(quote! {
        // TODO: Once Rust supports `unsafe(derive(...))`, make sure that's
        // required here.
        unsafe impl #impl_generics ::fromsoftware_shared::Superclass for #superclass
            #ty_generics #where_clause
        {
            fn vmt_rva() -> ::pelite::pe64::Rva {
                crate::rva::get().#rva
            }
        }

        #subclass_enums
    }))
}

/// Extracts the `#[superclass(children(...))]` attribute from
/// [superclass_struct] and uses it to build enums that require knowledge of all
/// the subclasses.
fn build_subclass_enums(superclass_struct: &mut ItemStruct) -> Result<TokenStream2> {
    let subclasses = extract_subclasses(superclass_struct)?;
    if subclasses.is_empty() {
        return Ok(TokenStream2::new());
    }

    let superclass = &superclass_struct.ident;
    let (impl_generics, _, _) = superclass_struct.generics.split_for_impl();

    let enum_name = format_ident!("{}Subclass", superclass);
    let mut_enum_name = format_ident!("{}SubclassMut", superclass);
    let lifetime = new_lifetime("sub", Span::call_site());
    let generics_with_lifetime = add_lifetime(&lifetime, &superclass_struct.generics);
    let (impl_generics_with_lifetime, ty_generics_with_lifetime, where_clause) =
        generics_with_lifetime.split_for_impl();

    let vis = &superclass_struct.vis;
    let type_param: TypeParam =
        syn::parse2(quote! { __Subclass: ::fromsoftware_shared::Subclass<#superclass> })?;
    let generics_with_subclass = add_type_param(&type_param, &generics_with_lifetime);
    let (impl_generics_with_subclass, _, _) = generics_with_subclass.split_for_impl();
    Ok(quote! {
        /// An enum of all known subclasses of [#superclass].
        #[derive(::std::marker::Copy, ::std::clone::Clone)]
        #vis enum #impl_generics #enum_name #ty_generics_with_lifetime #where_clause {
            #(
                #subclasses(&#lifetime #subclasses)
            ),*
            , #superclass(&#lifetime #superclass)
        }

        /// A mutable enum of all known subclasses of [#superclass].
        #vis enum #impl_generics #mut_enum_name #ty_generics_with_lifetime #where_clause {
            #(
                #subclasses(&#lifetime mut #subclasses)
            ),*
            , #superclass(&#lifetime mut #superclass)
        }

        impl #impl_generics_with_lifetime #enum_name #ty_generics_with_lifetime #where_clause {
            /// Returns this as a [#superclass].
            #vis fn superclass(&self) -> &#superclass {
                match self {
                    #(
                        #enum_name::#subclasses(subclass) => subclass.superclass()
                    ),*
                    , #enum_name::#superclass(superclass) => superclass
                }
            }
        }

        impl #impl_generics_with_lifetime #mut_enum_name #ty_generics_with_lifetime #where_clause {
            /// Returns this as a [#superclass].
            #vis fn superclass(&self) -> &#superclass {
                match self {
                    #(
                        #mut_enum_name::#subclasses(subclass) => subclass.superclass()
                    ),*
                    , #mut_enum_name::#superclass(superclass) => superclass
                }
            }

            /// Returns this as a mutable [#superclass].
            #vis fn superclass_mut(&mut self) -> &mut #superclass {
                match self {
                    #(
                        #mut_enum_name::#subclasses(subclass) => subclass.superclass_mut()
                    ),*
                    , #mut_enum_name::#superclass(superclass) => superclass
                }
            }
        }

        impl #impl_generics_with_lifetime From<#mut_enum_name #ty_generics_with_lifetime>
            for #enum_name #ty_generics_with_lifetime #where_clause
        {
            fn from(mutable: #mut_enum_name #ty_generics_with_lifetime)
                    -> #enum_name #ty_generics_with_lifetime {
                match mutable {
                    #(
                        #mut_enum_name::#subclasses(subclass) => #enum_name::#subclasses(subclass)
                    ),*
                    , #mut_enum_name::#superclass(superclass) => #enum_name::#superclass(superclass)
                }
            }
        }

        impl #impl_generics_with_subclass From<&#lifetime __Subclass> for #enum_name
            #ty_generics_with_lifetime #where_clause
        {
            fn from(subclass: &#lifetime __Subclass) -> #enum_name #ty_generics_with_lifetime {
                use ::fromsoftware_shared::Program;
                use ::pelite::pe64::Pe;

                // Converting the runtime VMT to an RVA saves a few pointer
                // calculations relative to converting each static RVA to a VA.
                let rva = ::fromsoftware_shared::Program::current()
                    .va_to_rva(subclass.superclass().vmt())
                    .unwrap_or(0);

                // Safety: We require that VMTs indicate object type.
                unsafe {
                    #(
                        if rva == #subclasses::vmt_rva() {
                            #enum_name::#subclasses(
                                ::std::ptr::NonNull::from_ref(subclass)
                                    .cast::<#subclasses>()
                                    .as_ref()
                            )
                        }
                    )else*
                    else {
                        #enum_name::#superclass(subclass.superclass())
                    }
                }
            }
        }

        impl #impl_generics_with_subclass From<&#lifetime mut __Subclass> for #mut_enum_name
            #ty_generics_with_lifetime #where_clause
        {
            fn from(subclass: &#lifetime mut __Subclass) -> #mut_enum_name #ty_generics_with_lifetime {
                use ::fromsoftware_shared::Program;
                use ::pelite::pe64::Pe;

                // Converting the runtime VMT to an RVA saves a few pointer
                // calculations relative to converting each static RVA to a VA.
                let rva = ::fromsoftware_shared::Program::current()
                    .va_to_rva(subclass.superclass().vmt())
                    .unwrap_or(0);

                // Safety: We require that VMTs indicate object type.
                unsafe {
                    #(
                        if rva == #subclasses::vmt_rva() {
                            #mut_enum_name::#subclasses(
                                ::std::ptr::NonNull::from_ref(subclass)
                                    .cast::<#subclasses>()
                                    .as_mut()
                            )
                        }
                    )else*
                    else {
                        #mut_enum_name::#superclass(subclass.superclass_mut())
                    }
                }
            }
        }
    })
}

/// Returns the types of the subclasses, determined by the
/// `#[superclass(children(...))]` attribute.
fn extract_subclasses(superclass: &mut ItemStruct) -> Result<Vec<TypePath>> {
    superclass
        .attrs
        .extract_if(.., |attr| attr.path().is_ident("superclass"))
        .flat_map(|attr| {
            let mut subclasses = Vec::new();
            if let Err(err) = attr.parse_nested_meta(|meta| {
                if !meta.path.is_ident("children") {
                    return Err(meta.error("unrecognized attribute"));
                }

                let args;
                parenthesized!(args in meta.input);
                for subclass in Punctuated::<TypePath, Token![,]>::parse_terminated(&args)? {
                    subclasses.push(Ok(subclass));
                }
                Ok(())
            }) {
                vec![Err(err)]
            } else {
                subclasses
            }
        })
        .collect::<Result<Vec<TypePath>>>()
}
