extern crate proc_macro;

use proc_macro2::TokenStream;

use syn::{
    punctuated::Punctuated, spanned::Spanned, AttributeArgs, Error, Ident, Item, Lit,
    Meta, MetaNameValue, Result,
};

/*
TODO:
  * Document that functions generated using this wrapper must us LinkObject as their
    argument / return value method.
*/

#[proc_macro_attribute]
pub fn wolfram_library_function(
    attr_args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = syn::parse_macro_input!(attr_args as AttributeArgs);
    let item = TokenStream::from(item);

    let output: TokenStream = match wolfram_library_function_impl(attr_args, item) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(output)
}

struct Options {
    /// The name to give the generated wrapper function. This is the name used as the
    /// 2nd argument of `LoadLibraryFunction`.
    name: Option<Ident>,
}

enum ArgumentsMode {
    ExprList,
    /// This contains a the pattern tokens from the `#[pattern(..)]` attribute
    PatternMatches {
        pattern: TokenStream,
        pattern_parameters: Vec<(Ident, syn::Type)>,
    },
}

struct Function {
    item: syn::ItemFn,

    name: Ident,

    arguments_mode: ArgumentsMode,
}

fn wolfram_library_function_impl(
    attr_args: AttributeArgs,
    item: TokenStream,
) -> Result<TokenStream> {
    let options = parse_attributes(attr_args)?;
    let function = Function::from_item(syn::parse2(item.clone())?)?;

    let wrapper_function_name = match options.name {
        Some(name) => name,
        None => Ident::new(
            &format!("{}_wrapper", function.name),
            proc_macro2::Span::call_site(),
        ),
    };

    if wrapper_function_name == function.name {
        return Err(Error::new(
            function.name.span(),
            "this name must be different from the value of the `name` attribute",
        ));
    }

    let tokens = match function.arguments_mode {
        ArgumentsMode::ExprList => {
            gen_arg_mode_expr_list(&function.item, function.name, wrapper_function_name)
        },
        ArgumentsMode::PatternMatches {
            ref pattern,
            ref pattern_parameters,
        } => gen_arg_mode_pattern(
            &function,
            wrapper_function_name,
            &pattern,
            &pattern_parameters,
        ),
    };

    Ok(tokens)
}

fn gen_arg_mode_expr_list(
    fn_item: &syn::ItemFn,
    function_name: Ident,
    wrapper_function_name: Ident,
) -> TokenStream {
    quote::quote! {
        #fn_item

        #[no_mangle]
        pub extern "C" fn #wrapper_function_name(
            libdata: ::wl_library_link::sys::WolframLibraryData,
            unsafe_link: ::wl_library_link::wstp::sys::WSLINK,
        ) -> std::os::raw::c_uint {
            ::wl_library_link::call_wstp_wolfram_library_function_expr_list(
                libdata,
                unsafe_link,
                #function_name
            )
        }
    }
}

fn gen_arg_mode_pattern(
    function: &Function,
    wrapper_function_name: Ident,
    pattern: &TokenStream,
    pattern_parameters: &Vec<(Ident, syn::Type)>,
) -> TokenStream {
    let fn_item = &function.item;
    let function_name = &function.name;

    let struct_name = quote::format_ident!("ArgumentsFor_{}", function_name);

    let parameter_names = pattern_parameters
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    let parameter_pairs = pattern_parameters
        .iter()
        .map(|(name, ty)| quote::quote! { #name: #ty, })
        .collect::<Vec<_>>();

    quote::quote! {
        #fn_item

        #[derive(wl_expr::FromExpr)]
        #[pattern({ #pattern })]
        #[allow(non_camel_case_types)]
        struct #struct_name {
            #(#parameter_pairs)*
        }

        #[no_mangle]
        pub extern "C" fn #wrapper_function_name(
            libdata: ::wl_library_link::sys::WolframLibraryData,
            unsafe_link: ::wl_library_link::wstp::sys::WSLINK,
        ) -> std::os::raw::c_uint {
            use ::wl_expr::{Expr, forms::{FromExpr, FormError}};
            use ::wl_library_link::WolframEngine;

            ::wl_library_link::call_wstp_wolfram_library_function(
                libdata,
                unsafe_link,
                |engine: &WolframEngine, argument_expr: Expr| -> Expr {
                    // `argument_expr` should have the head `List`, due to how LibraryFunction[]
                    // is implemented.
                    let args = match <#struct_name as FromExpr>::from_expr(&argument_expr) {
                        Ok(args) => args,
                        Err(err) => return Expr! {
                            Failure["ArgumentShape", <|
                                "Message" -> %[format!("{}", FormError::from(err))]
                            |>]
                        },
                    };

                    #function_name(engine, #( args.#parameter_names ),*)
                }
            )
        }
    }
}

fn parse_attributes(attr_args: AttributeArgs) -> Result<Options> {
    use syn::NestedMeta;

    let mut name_option: Option<Ident> = None;

    for attr in attr_args {
        let MetaNameValue {
            path,
            eq_token: _,
            lit,
        } = match attr {
            NestedMeta::Meta(Meta::NameValue(nv)) => nv,
            _ => {
                return Err(Error::new(
                    attr.span(),
                    "expected `name = \"..\" attribute`",
                ))
            },
        };

        if !path.is_ident("name") {
            return Err(Error::new(
                path.span(),
                "expected `name = \"..\"` attribute",
            ));
        }

        // Verify that we have not already parsed a value for the `name` option. E.g,
        // prevent `#[wolfram_library_function(name = "name1", name = "name2")]`.
        if name_option.is_some() {
            return Err(Error::new(path.span(), "attribute appears more than once"));
        }

        debug_assert!(name_option == None);
        name_option = Some(parse_name_option_value(lit)?);
    }

    Ok(Options { name: name_option })
}

fn parse_name_option_value(lit: syn::Lit) -> Result<Ident> {
    let litstr = match lit {
        Lit::Str(string) => string,
        _ => return Err(Error::new(lit.span(), "expected string literal")),
    };

    let name_ident: Ident = match syn::parse_str::<Ident>(&litstr.value()) {
        Ok(ident) => ident,
        Err(err) => {
            return Err(Error::new(
                litstr.span(),
                format!("string is not a valid identifier: {}", err),
            ))
        },
    };

    Ok(name_ident)
}

impl Function {
    fn from_item(item: Item) -> Result<Self> {
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
