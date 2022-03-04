use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{spanned::Spanned, Error, Ident, Item, Meta, NestedMeta};

//======================================
// #[wolfram_library_link::export]
//======================================

// NOTE: The comment below was written for the `export![]` declarative macro. Parts of
//       it are still relevent to the `#[export]` attribute procedural macro
//       implementation, but some of the constraints are not relevant any more.
//       I've kept it for context about what the tradeoffs in this functionality are.

// # Design constraints
//
// The current design of this macro is intended to accommodate the following constraints:
//
// 1. Support automatic generation of wrapper functions without using procedural macros,
//    and with minimal code duplication. Procedural macros require external dependencies,
//    and can significantly increase compile times.
//
//      1a. Don't depend on the entire function definition to be contained within the
//          macro invocation, which leads to unergonomic rightward drift. E.g. don't
//          require something like:
//
//          export![
//              fn foo(x: i64) { ... }
//          ]
//
//      1b. Don't depend on the entire function declaration to be repeated in the
//          macro invocation. E.g. don't require:
//
//              fn foo(x: i64) -> i64 {...}
//
//              export![
//                  fn foo(x: i64) -> i64;
//              ]
//
// 2. The name of the function in Rust should match the name of the function that appears
//    in the WL LibraryFunctionLoad call. E.g. needing different `foo` and `foo__wrapper`
//    named must be avoided.
//
// To satisfy constraint 1, it's necessary to depend on the type system rather than
// clever macro operations. This leads naturally to the creation of the `NativeFunction`
// trait, which is implemented for all suitable `fn(..) -> _` types.
//
// Constraint 1b is unable to be met completely by the current implementation due to
// limitations with Rust's coercion from `fn(A, B, ..) -> C {some_name}` to
// `fn(A, B, ..) -> C`. The coercion requires that the number of parameters (`foo(_, _)`)
// be made explicit, even if their types can be elided. If eliding the number of fn(..)
// arguments were permitted, `export![foo]` could work.
//
// To satisfy constraint 2, this implementation creates a private module with the same
// name as the function that is being wrapped. This is required because in Rust (as in
// many languages), it's illegal for two different functions with the same name to exist
// within the same module:
//
// ```
// fn foo { ... }
//
// #[no_mangle]
// pub extern "C" fn foo { ... } // Error: conflicts with the other foo()
// ```
//
// This means that the export![] macro cannot simply generate a wrapper function
// with the same name as the wrapped function, because they would conflict.
//
// However, it *is* legal for a module to contain a function and a child module that
// have the same name. Because `#[no_mangle]` functions are exported from the crate no
// matter where they appear in the module heirarchy, this offers an effective workaround
// for the name clash issue, while satisfy constraint 2's requirement that the original
// function and the wrapper function have the same name:
//
// ```
// fn foo() { ... } // This does not conflict with the `foo` module.
//
// mod foo {
//     #[no_mangle]
//     pub extern "C" fn foo(..) { ... } // This does not conflict with super::foo().
// }
// ```
pub(crate) fn export(
    attrs: syn::AttributeArgs,
    item: TokenStream,
) -> Result<TokenStream2, Error> {
    //----------------------------------------------------
    // Parse the `#[export(<attrs>)]` attribute arguments.
    //----------------------------------------------------

    let ExportArgs { exported_name } = parse_export_attribute_args(attrs)?;

    //--------------------------------------------------------------------
    // Validate that this attribute was applied to a `fn(..) { .. }` item.
    //--------------------------------------------------------------------

    let item: Item = syn::parse(item)?;

    let func = match item {
        Item::Fn(func) => func,
        _ => {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "this attribute can only be applied to `fn(..) {..}` items",
            ));
        },
    };

    //-------------------------
    // Validate the user `func`
    //-------------------------

    // No `async`
    if let Some(async_) = func.sig.asyncness {
        return Err(Error::new(
            async_.span(),
            "exported function cannot be `async`",
        ));
    }

    // No generics
    if let Some(lt) = func.sig.generics.lt_token {
        return Err(Error::new(lt.span(), "exported function cannot be generic"));
    }

    //----------------------------
    // Create the output function.
    //----------------------------

    let name = func.sig.ident.clone();
    let exported_name: Ident = match exported_name {
        Some(name) => name,
        None => func.sig.ident.clone(),
    };

    let params = func.sig.inputs.clone();
    let params = vec![quote! { _ }; params.len()];

    let output = quote! {
        // Include the users function in the output unchanged.
        #func

        mod #name {
            #[no_mangle]
            pub unsafe extern "C" fn #exported_name(
                lib: ::wolfram_library_link::sys::WolframLibraryData,
                argc: ::wolfram_library_link::sys::mint,
                args: *mut ::wolfram_library_link::sys::MArgument,
                res: ::wolfram_library_link::sys::MArgument,
            ) -> std::os::raw::c_uint {
                // Cast away the unique `fn(...) {some_name}` function type to get the
                // generic `fn(...)` type.
                let func: fn(#(#params),*) -> _ = super::#name;

                ::wolfram_library_link::macro_utils::call_native_wolfram_library_function(
                    lib,
                    args,
                    argc,
                    res,
                    func
                )
            }
        }

        // Register this exported function.
        ::wolfram_library_link::inventory::submit! {
            ::wolfram_library_link::macro_utils::LibraryLinkFunction::Native {
                name: stringify!(#exported_name),
                signature: || {
                    let func: fn(#(#params),*) -> _ = #name;
                    let func: &dyn ::wolfram_library_link::NativeFunction<'_> = &func;

                    func.signature()
                }
            }
        }
    };

    Ok(output)
}

/// Attribute arguments recognized by the `#[export(...)]` macro.
struct ExportArgs {
    exported_name: Option<Ident>,
}

fn parse_export_attribute_args(attrs: syn::AttributeArgs) -> Result<ExportArgs, Error> {
    let mut exported_name: Option<Ident> = None;

    for attr in attrs {
        match attr {
            NestedMeta::Meta(ref meta) => match meta {
                Meta::List(_) | Meta::Path(_) => {
                    return Err(Error::new(
                        attr.span(),
                        "unrecognized export attribute argument",
                    ));
                },
                Meta::NameValue(syn::MetaNameValue {
                    path,
                    eq_token: _,
                    lit,
                }) => {
                    if path.is_ident("name") {
                        if exported_name.is_some() {
                            return Err(Error::new(
                                attr.span(),
                                "duplicate definition for `name`",
                            ));
                        }

                        let lit_str = match lit {
                            syn::Lit::Str(str) => str,
                            _ => {
                                return Err(Error::new(
                                    lit.span(),
                                    "expected `name = \"...\"`",
                                ))
                            },
                        };

                        exported_name = Some(
                            lit_str
                                .parse::<Ident>()
                                // Use the correct span for this error.
                                .map_err(|err| Error::new(lit_str.span(), err))?,
                        );
                    } else {
                        return Err(Error::new(
                            path.span(),
                            "unrecognized export attribute named argument",
                        ));
                    }
                },
            },
            NestedMeta::Lit(_) => {
                return Err(Error::new(
                    attr.span(),
                    "unrecognized export attribute literal argument",
                ));
            },
        }
    }

    Ok(ExportArgs { exported_name })
}
