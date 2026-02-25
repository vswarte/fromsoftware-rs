use std::collections::BTreeSet;

use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn stepper_states_helper(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse(input)?;
    let input_struct_ident = &input.ident;

    let Data::Enum(e) = &input.data else {
        return Err(Error::new_spanned(
            &input.ident,
            "StepperStates can only be derived on enums",
        ));
    };

    validate_stepper_enum_storage(&input)?;
    validate_stepper_enum_variants(e)?;

    let count = e.variants.len();
    let expanded = quote! {
        unsafe impl ::fromsoftware_shared::StepperStates for #input_struct_ident {
            type StepperFnArray<TStepperFn> = [TStepperFn; #count];
        }
    };

    Ok(TokenStream::from(expanded))
}

fn validate_stepper_enum_storage(i: &DeriveInput) -> Result<()> {
    let Some(repr_attr) = i.attrs.iter().find(|a| a.path().is_ident("repr")) else {
        return Err(Error::new_spanned(
            &i.ident,
            "Enum must apply a #[repr(i32)], there is currently no repr specified at all",
        ));
    };

    let Meta::List(repr_args) = &repr_attr.meta else {
        return Err(Error::new_spanned(
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
        return Err(Error::new_spanned(
            &i.ident,
            "Enum must apply a #[repr(i32)]",
        ));
    }

    Ok(())
}

fn validate_stepper_enum_variants(e: &DataEnum) -> Result<()> {
    let mut values = BTreeSet::<i32>::new();

    if e.variants.len() < 2 {
        return Err(Error::new_spanned(
            e.enum_token,
            "Stepper states enum must define at least `NotExecuting = -1` and one active state",
        ));
    }

    for v in &e.variants {
        if !matches!(v.fields, Fields::Unit) {
            return Err(Error::new_spanned(&v.ident, "All states must be unit"));
        }

        let Some((_, expr)) = &v.discriminant else {
            return Err(Error::new_spanned(
                &v.ident,
                "All states must have explicit discriminants (e.g. `GuestInviteWait = 3`)",
            ));
        };

        let val = read_i32_lit(expr)?;
        if val < -1 {
            return Err(Error::new_spanned(
                &v.ident,
                "Discriminant cannot be a negative unless it's the `NotExecuting` state",
            ));
        }

        if val == -1 && v.ident != "NotExecuting" {
            return Err(Error::new_spanned(
                &v.ident,
                "Only `NotExecuting` may use discriminant -1",
            ));
        }

        values.insert(val);
    }

    if !values.contains(&-1) {
        return Err(Error::new_spanned(
            e.enum_token,
            "Missing NotExecuting state with discriminant -1",
        ));
    }

    let min = *values.first().unwrap();
    let max = *values.last().unwrap();
    if (max - min + 1) as usize != values.len() {
        let missing: Vec<i32> = (min..=max).filter(|x| !values.contains(x)).collect();

        return Err(Error::new_spanned(
            e.enum_token,
            format!("Discriminants contain gaps; missing values: {missing:?}"),
        ));
    }

    Ok(())
}

fn read_i32_lit(expr: &Expr) -> Result<i32> {
    fn parse_i32(expr: &Expr) -> Result<i32> {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) => i
                .base10_parse::<i32>()
                .map_err(|_| Error::new_spanned(expr, "Discriminant out of i32 range")),
            _ => Err(Error::new_spanned(
                expr,
                "Use an integer literal like -1 or 3",
            )),
        }
    }

    match expr {
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr: inner,
            ..
        }) => {
            let v = parse_i32(inner)?;
            v.checked_neg()
                .ok_or_else(|| Error::new_spanned(expr, "Discriminant out of i32 range"))
        }
        _ => parse_i32(expr),
    }
}
