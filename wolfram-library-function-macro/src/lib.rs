extern crate proc_macro;

use proc_macro2::TokenStream;

use syn::{punctuated::Punctuated, spanned::Spanned, Item, Error, Result};

/*
TODO:
  * needs to check that the function is marked `pub`
  * Document that functions generated using this wrapper must us LinkObject as their
    argument / return value method.
*/

#[proc_macro_attribute]
pub fn wolfram_library_function(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);

    let output: TokenStream = match wolfram_library_function_impl(attr, item) {
        Ok(stream) => stream,
        Err(err) => err.to_compile_error(),
    };

    proc_macro::TokenStream::from(output)
}

fn wolfram_library_function_impl(
    attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream> {
    let fnitem = syn::parse2(item.clone())?;

    let function_name = validate_function(&fnitem)?;
    let wrapper_function_name = syn::Ident::new(
        &format!("{}_wrapper", function_name),
        proc_macro2::Span::call_site(),
    );

    let tokens = quote::quote! {
        #fnitem

        #[no_mangle]
        pub extern "C" fn #wrapper_function_name(
            libdata: ::wl_library_link::sys::WolframLibraryData,
            unsafe_link: ::wl_library_link::wstp::sys::WSLINK,
        ) -> std::os::raw::c_uint {
            ::wl_library_link::call_wolfram_library_function(
                libdata,
                unsafe_link,
                #function_name
            )
        }
    };

    Ok(tokens)
}

fn validate_function(item: &Item) -> Result<syn::Ident> {
    let fnitem = match item {
        Item::Fn(fnitem) => fnitem,
        _ => {
            return Err(Error::new(
                item.span(),
                "`wolfram_library_function` attribute can only be used on functions",
            ))
        },
    };

    let syn::ItemFn {
        attrs: _,
        vis,
        sig,
        block: _,
    } = fnitem;

    // Ensure that the function is marked `pub`.
    let _: () = check_visibility(vis, sig)?;

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

    let _: () = check_parameters(&sig.inputs, sig.paren_token)?;

    Ok(sig.ident.clone())
}

/// Ensure that the function is marked `pub`.
fn check_visibility(
    vis: &syn::Visibility,
    sig: &syn::Signature,
) -> Result<()> {
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

fn check_parameters(
    inputs: &Punctuated<syn::FnArg, syn::token::Comma>,
    parens: syn::token::Paren,
) -> Result<()> {
    if inputs.len() != 2 {
        return Err(Error::new(
            parens.span,
            "Wolfram library function must have 2 parameters",
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
