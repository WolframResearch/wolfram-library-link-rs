use syn::{punctuated::Punctuated, spanned::Spanned, Error, Ident, Item, Result};

use crate::ArgumentsMode;

pub(crate) struct Function {
    pub item: syn::ItemFn,

    pub name: Ident,

    pub arguments_mode: ArgumentsMode,
}

impl Function {
    pub fn from_item(item: Item) -> Result<Self> {
        let mut fn_item =
            match item {
                Item::Fn(fnitem) => fnitem,
                _ => return Err(Error::new(
                    item.span(),
                    "`wolfram_library_function` attribute can only be used on functions",
                )),
            };

        let arguments_mode = determine_arguments_mode(&mut fn_item.attrs, &fn_item.sig)?;

        let _: () = validate_function(&fn_item, &arguments_mode)?;

        Ok(Function {
            name: fn_item.sig.ident.clone(),
            item: fn_item,
            arguments_mode,
        })
    }
}

fn parse_pattern_parameters(sig: &syn::Signature) -> Result<Vec<(Ident, syn::Type)>> {
    let parameters = sig
        .inputs
        .iter()
        // Skip the &WolframEngine parameter
        .skip(1)
        .map(|arg| {
            let typed = match arg {
                // `&self`
                syn::FnArg::Receiver(_) => {
                    return Err(Error::new(arg.span(), "expected non-self parameter"))
                },
                syn::FnArg::Typed(typed) => typed,
            };

            let name = match &*typed.pat {
                // TODO: Check if this `pat_ident` has any attributes, like the planned
                //       #[list] and #[sequence] attributes.
                syn::Pat::Ident(pat_ident)
                    if pat_ident.subpat.is_none() && pat_ident.by_ref.is_none() =>
                {
                    pat_ident.ident.clone()
                },
                _ => return Err(Error::new(
                    typed.pat.span(),
                    "Wolfram library function expects parameters to be plain identifiers",
                )),
            };

            Ok((name, (*typed.ty).clone()))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(parameters)
}

fn determine_arguments_mode(
    attrs: &mut Vec<syn::Attribute>,
    sig: &syn::Signature,
) -> Result<ArgumentsMode> {
    let mut pattern: Option<(usize, syn::Attribute)> = None;
    for (index, attr) in attrs.iter().enumerate() {
        if !attr.path.is_ident("pattern") {
            continue;
        }

        if pattern.is_some() {
            return Err(Error::new(attr.span(), "attribute appears more than once"));
        }

        pattern = Some((index, attr.clone()));
    }

    match pattern {
        Some((index, attr)) => {
            use proc_macro2::{Delimiter, TokenTree};

            // Remove the `#[pattern(..)]` attribute from the function item.
            attrs.remove(index);

            // Get get tokens inside the parenthesis in `#[pattern(...)]`.
            let attr_span = attr.span();
            let tokens: Vec<TokenTree> = attr.tokens.into_iter().collect();

            let inner_tokens = match tokens.as_slice() {
                &[TokenTree::Group(ref group)]
                    if group.delimiter() == Delimiter::Parenthesis =>
                {
                    group.stream()
                },
                // E.g. `#[pattern = "..."]`
                _ => {
                    return Err(Error::new(
                        attr_span,
                        "expected attribute with format `#[pattern(..)]`",
                    ))
                },
            };

            let pattern_parameters = parse_pattern_parameters(sig)?
                .into_iter()
                .collect::<Vec<_>>();

            Ok(ArgumentsMode::PatternMatches {
                pattern: inner_tokens,
                pattern_parameters,
            })
        },
        None => Ok(ArgumentsMode::ExprList),
    }
}

fn validate_function(fnitem: &syn::ItemFn, args_mode: &ArgumentsMode) -> Result<()> {
    let syn::ItemFn {
        attrs: _,
        vis,
        sig,
        block: _,
    } = fnitem;

    // Ensure that the function is marked `pub`.
    let _: () = validate_visibility(vis, sig)?;

    // Ensure that the function is not marked with `const` or `async`
    if let Some(const_) = sig.constness {
        return Err(Error::new(
            const_.span,
            "Wolfram library function must not be `const`",
        ));
    }
    if let Some(async_) = sig.asyncness {
        return Err(Error::new(
            async_.span,
            "Wolfram library function must not be `async`",
        ));
    }

    // Ensure that the function is using the native Rust ABI (and *not* e.g.
    // `extern "C"`).
    if let Some(abi) = &sig.abi {
        return Err(Error::new(
            abi.span(),
            "Wolfram library function must use the native Rust ABI",
        ));
    }

    // Ensure that the function is not generic
    if sig.generics.params.len() > 0 {
        return Err(Error::new(
            sig.generics.params.span(),
            "Wolfram library function must not be generic",
        ));
    }

    // Ensure that the function does not have variadic arguments, e.g. `args: ..i32`
    if let Some(variadic) = &sig.variadic {
        return Err(Error::new(
            variadic.span(),
            "Wolfram library function must not be variadic",
        ));
    }

    let _: () = validate_parameters(&sig.inputs, sig.paren_token, &args_mode)?;

    Ok(())
}

/// Ensure that the function is marked `pub`.
fn validate_visibility(vis: &syn::Visibility, sig: &syn::Signature) -> Result<()> {
    match vis {
        // `pub fn name()`
        syn::Visibility::Public(_) => Ok(()),
        // `pub(crate) fn name()`
        syn::Visibility::Restricted(restriction) => {
            return Err(Error::new(
                restriction.paren_token.span,
                "Wolfram library function must be marked `pub`, with no restrictions",
            ))
        },
        // `crate fn name()`
        syn::Visibility::Crate(_) => {
            return Err(Error::new(
                vis.span(),
                "Wolfram library function must be marked `pub`",
            ))
        },
        // `fn name()`
        // Same as the ::Crate case, but the error span is the `fn` token
        syn::Visibility::Inherited => {
            return Err(Error::new(
                sig.fn_token.span(),
                "Wolfram library function must be marked `pub`",
            ))
        },
    }
}

fn validate_parameters(
    inputs: &Punctuated<syn::FnArg, syn::token::Comma>,
    parens: syn::token::Paren,
    args_mode: &ArgumentsMode,
) -> Result<()> {
    if inputs.is_empty() {
        return Err(Error::new(
            parens.span,
            "Wolfram library function must have at least 1 parameter for `&WolframEngine`",
        ));
    }

    //
    // Check that the first parameter is `&WolframEngine`
    //

    let first_param =
        match &inputs[0] {
            // `self` OR `&self`
            syn::FnArg::Receiver(receiver) => return Err(Error::new(
                receiver.span(),
                "First parameter of Wolfram library function must be `&WolframEngine`",
            )),
            syn::FnArg::Typed(pat_type) => pat_type,
        };

    match args_mode {
        ArgumentsMode::ExprList => (),
        ArgumentsMode::PatternMatches { .. } => return Ok(()),
    }

    //
    // Check that there are 2 parameters, and that the 2nd one is `Vec<Expr>`.
    //

    if inputs.len() != 2 {
        return Err(Error::new(
            parens.span,
            "Wolfram library function must have 2 parameters",
        ));
    }

    if !first_param.attrs.is_empty() {
        return Err(Error::new(
            first_param.attrs[0].span(),
            "Unknown Wolfram library function attribute",
        ));
    }

    match &*first_param.ty {
        syn::Type::Reference(reference) => {
            let syn::TypeReference {
                and_token: _,
                lifetime,
                mutability,
                elem: _,
            } = reference;

            // TODO(!): Test the *type* error you get if the parameter is not the
            //          WolframEngine type, and see if it's sufficiently helpful to make
            //          this check unnecessary.
            // let path = match &**elem {
            //     syn::Type::Path(path) => path,
            //     _ => {
            //         return Err(Error::new(
            //             first_param.ty.span(),
            //             "Expected type `&WolframEngine`",
            //         ))
            //     },
            // };
            // let path_str = quote::quote!(#path).to_string();
            // if !(path_str == "WolframEngine" || path_str == "wl_library_link :: WolframEngine") {
            //     return Err(Error::new(first_param.ty.span(), "Expected type `&WolframEngine`"))
            // }

            if let Some(lifetime) = lifetime {
                return Err(Error::new(
                    lifetime.span(),
                    "Explicit lifetime is not allowed within a Wolfram library function",
                ));
            }

            if let Some(mut_) = mutability {
                return Err(Error::new(
                    mut_.span(),
                    "Wolfram library function engine parameter must be taken by immutable `&` reference",
                ));
            }
        },
        _ => {
            return Err(Error::new(
                first_param.ty.span(),
                "First parameter of Wolfram library function must be `&WolframEngine`",
            ))
        },
    }

    //
    // TODO?: Check that the second parameter is `Vec<Expr>`
    //

    Ok(())
}
