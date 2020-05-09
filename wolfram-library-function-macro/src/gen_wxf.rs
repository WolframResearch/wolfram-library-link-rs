use proc_macro2::TokenStream;
use syn::Ident;

use crate::Function;

pub(crate) fn gen_arg_mode_expr_list<'a>(
    fn_item: &'a syn::ItemFn,
    function_name: Ident,
    wrapper_function_name: Ident,
) -> TokenStream {
    let inner = quote::quote! {
        ::wl_library_link::call_wxf_wolfram_library_function_expr_list(
            libdata,
            wxf_argument,
            wxf_result,
            #function_name
        )
    };

    gen_wxf_function(fn_item, wrapper_function_name, inner)
}

pub(crate) fn gen_arg_mode_pattern<'a>(
    function: &'a Function,
    wrapper_function_name: Ident,
    pattern: &'a TokenStream,
    pattern_parameters: &'a Vec<(Ident, syn::Type)>,
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

    let inner = quote::quote! {
        use ::wl_expr::{Expr, forms::{FromExpr, FormError}};
        use ::wl_library_link::WolframEngine;

        ::wl_library_link::call_wxf_wolfram_library_function(
            libdata,
            wxf_argument,
            wxf_result
            |engine: &WolframEngine, argument_expr: Expr| -> Expr {
                #[derive(wl_expr::FromExpr)]
                #[pattern({ #pattern })]
                #[allow(non_camel_case_types)]
                struct #struct_name {
                    #(#parameter_pairs)*
                }

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
    };

    gen_wxf_function(fn_item, wrapper_function_name, inner)
}

fn gen_wxf_function(
    fn_item: &syn::ItemFn,
    wrapper_function_name: Ident,
    inner: TokenStream,
) -> TokenStream {
    quote::quote! {
        #fn_item

        #[no_mangle]
        pub extern "C" fn #wrapper_function_name(
            libdata: ::wl_library_link::sys::WolframLibraryData,
            argc: ::wl_library_link::sys::mint,
            args: *mut ::wl_library_link::sys::MArgument,
            wxf_result: ::wl_library_link::sys::MArgument,
        ) -> std::os::raw::c_uint {
            if argc != 1 {
                return ::wl_library_link::sys::LIBRARY_FUNCTION_ERROR;
            }

            // Take the first argument, at offset 0 in `args`.
            let wxf_argument = unsafe { *args };

            #inner
        }
    }
}
