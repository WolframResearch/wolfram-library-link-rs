use syn::{Ident};
use proc_macro2::TokenStream;

use crate::Function;

pub(crate) fn gen_arg_mode_expr_list(
    fn_item: &syn::ItemFn,
    function_name: Ident,
    wrapper_function_name: Ident,
) -> TokenStream {
    let inner = quote::quote! {
        ::wl_library_link::call_wstp_wolfram_library_function_expr_list(
            libdata,
            unsafe_link,
            #function_name
        )
    };

    gen_wstp_function(fn_item, wrapper_function_name, inner)
}

pub(crate) fn gen_arg_mode_pattern(
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

    let inner = quote::quote! {
        use ::wl_expr::{Expr, forms::{FromExpr, FormError}};
        use ::wl_library_link::WolframEngine;

        ::wl_library_link::call_wstp_wolfram_library_function(
            libdata,
            unsafe_link,
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

    gen_wstp_function(fn_item, wrapper_function_name, inner)
}

fn gen_wstp_function(
    fn_item: &syn::ItemFn,
    wrapper_function_name: Ident,
    inner: TokenStream,
) -> TokenStream {
    quote::quote! {
        #fn_item

        #[no_mangle]
        pub extern "C" fn #wrapper_function_name(
            libdata: ::wl_library_link::sys::WolframLibraryData,
            unsafe_link: ::wl_library_link::wstp::sys::WSLINK,
        ) -> std::os::raw::c_uint {
            #inner
        }
    }
}
