use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn::{spanned::Spanned, Error, Item};

//======================================
// #[wolfram_library_link::init]
//======================================

#[proc_macro_attribute]
pub fn init(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match init_(attr.into(), item) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn init_(attr: TokenStream2, item: TokenStream) -> Result<TokenStream2, Error> {
    // Validate that we got `#[init]` and not `#[init(some, unexpected, arguments)]`.
    if !attr.is_empty() {
        return Err(Error::new(attr.span(), "unexpected attribute arguments"));
    }

    //--------------------------------------------------------------------
    // Validate that this attribute was applied to a `fn(..) { .. }` item.
    //--------------------------------------------------------------------

    let item: Item = syn::parse(item)?;

    let func = match item {
        Item::Fn(func) => func,
        _ => {
            return Err(Error::new(
                attr.span(),
                "this attribute can only be applied to `fn(..) {..}` items",
            ))
        },
    };

    //-------------------------
    // Validate the user `func`
    //-------------------------

    // No `async`
    if let Some(async_) = func.sig.asyncness {
        return Err(Error::new(
            async_.span(),
            "initialization function cannot be `async`",
        ));
    }

    // No generics
    if let Some(lt) = func.sig.generics.lt_token {
        return Err(Error::new(
            lt.span(),
            "initialization function cannot be generic",
        ));
    }

    // No parameters
    if !func.sig.inputs.is_empty() {
        return Err(Error::new(
            func.sig.inputs.span(),
            "initialization function should have zero parameters",
        ));
    }

    //--------------------------------------------------------
    // Create the output WolframLibrary_initialize() function.
    //--------------------------------------------------------

    let user_init_fn_name: syn::Ident = func.sig.ident.clone();

    let output = quote! {
        #func

        #[no_mangle]
        pub unsafe extern "C" fn WolframLibrary_initialize(
            lib: ::wolfram_library_link::sys::WolframLibraryData,
        ) -> ::std::os::raw::c_int {
            ::wolfram_library_link::macro_utils::init_with_user_function(
                lib,
                #user_init_fn_name
            )
        }
    };

    Ok(output)
}
