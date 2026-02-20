use std::collections::BTreeSet;

use syn::{DataEnum, DeriveInput, Expr, ExprLit, ExprUnary, Fields, Lit, Meta, UnOp};

pub fn validate_stepper_enum_storage(i: &DeriveInput) -> syn::Result<()> {
    let Some(repr_attr) = i.attrs.iter().find(|a| a.path().is_ident("repr")) else {
        return Err(syn::Error::new_spanned(
            &i.ident,
            "Enum must apply a #[repr(i32)], there is currently no repr specified at all",
        ));
    };

    let Meta::List(repr_args) = &repr_attr.meta else {
        return Err(syn::Error::new_spanned(
            &i.ident,
            "Enum must apply a #[repr(i32)], the repr attribute currently has no arguments",
        ));
    };

    if !repr_args
        .tokens
        .to_string()
        .split(',')
        .map(|s| s.trim())
        .any(|s| s == "i32")
    {
        return Err(syn::Error::new_spanned(
            &i.ident,
            "Enum must apply a #[repr(i32)]",
        ));
    }

    Ok(())
}

pub fn validate_stepper_enum_variants(e: &DataEnum) -> syn::Result<()> {
    let mut values = BTreeSet::<i32>::new();

    for v in &e.variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                &v.ident,
                "All variants must be unit",
            ));
        }

        let Some((_, expr)) = &v.discriminant else {
            return Err(syn::Error::new_spanned(
                &v.ident,
                "All variants must have explicit discriminants (e.g. `GuestInviteWait = 3`)",
            ));
        };

        let val = read_i32_lit(expr)?;
        if val < 0 && val != -1 {
            return Err(syn::Error::new_spanned(
                &v.ident,
                "Disciminant cannot be a negative unless it's the Inactive state",
            ));
        }

        if !values.insert(val) {
            return Err(syn::Error::new_spanned(
                &v.ident,
                format!("Duplicate discriminant value {val}"),
            ));
        }
    }

    if !values.contains(&-1) {
        return Err(syn::Error::new_spanned(
            &e.variants[0].ident,
            "Missing Inactive variant with discriminant -1",
        ));
    }

    let non_sentinel: Vec<i32> = values.iter().copied().filter(|&x| x != -1).collect();
    if non_sentinel.is_empty() {
        return Err(syn::Error::new_spanned(
            &e.variants[0].ident,
            "Stepper states must have more states than just the Inactive variant (-1)",
        ));
    }

    let min = *non_sentinel.first().unwrap();
    let max = *non_sentinel.last().unwrap();

    let expected_len = (max - min + 1) as usize;
    if expected_len != non_sentinel.len() {
        let set: BTreeSet<i32> = non_sentinel.iter().copied().collect();
        let missing: Vec<i32> = (min..=max).filter(|x| !set.contains(x)).collect();

        return Err(syn::Error::new_spanned(
            &e.variants[0].ident,
            format!("Discriminants contain gaps; missing values: {missing:?}"),
        ));
    }

    Ok(())
}

fn read_i32_lit(expr: &Expr) -> syn::Result<i32> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(i), ..
        }) => i
            .base10_parse::<i32>()
            .map_err(|_| syn::Error::new_spanned(expr, "Discriminant out of i32 range")),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr: inner,
            ..
        }) => match inner.as_ref() {
            Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) => {
                let v = i
                    .base10_parse::<i32>()
                    .map_err(|_| syn::Error::new_spanned(inner, "Discriminant out of i32 range"))?;
                v.checked_neg()
                    .ok_or_else(|| syn::Error::new_spanned(expr, "Discriminant out of i32 range"))
            }
            _ => Err(syn::Error::new_spanned(
                expr,
                "Use an integer literal like -1 or 3",
            )),
        },
        _ => Err(syn::Error::new_spanned(
            expr,
            "Use an integer literal like -1 or 3",
        )),
    }
}
