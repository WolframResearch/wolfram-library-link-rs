#![cfg_attr(feature = "nightly", feature(proc_macro_diagnostic))]

mod function;
mod gen_wstp;
mod gen_wxf;

use proc_macro2::TokenStream;

use syn::{
    spanned::Spanned, AttributeArgs, Error, Ident, Lit, Meta, MetaNameValue, Result,
};

use self::function::Function;

/*
TODO:
  * Document that functions generated using this wrapper must us LinkObject as their
    argument / return value method.
*/

struct Options {
    /// The name to give the generated wrapper function. This is the name used as the
    /// 2nd argument of `LoadLibraryFunction`.
    name: Option<Ident>,
    protocol: Option<Protocol>,
}

#[derive(Debug, PartialEq)]
enum Protocol {
    /// Pass expressions using a WSTP LinkObject.
    WSTP,
    /// Pass expresions by serializing them to WXF and then deserializing them on the
    /// other end of the link.
    WXF,
}

enum ArgumentsMode {
    ExprList,
    /// This contains a the pattern tokens from the `#[pattern(..)]` attribute
    PatternMatches {
        pattern: TokenStream,
        pattern_parameters: Vec<(Ident, syn::Type)>,
    },
}

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

    // WSTP is the default protocol.
    // TODO: Change this default protocol if it turns out WXF is faster.
    let protocol = match options.protocol {
        Some(Protocol::WSTP) | None => Protocol::WSTP,
        Some(Protocol::WXF) => Protocol::WXF,
    };

    //
    // Generate the wrapper function
    //

    let tokens = match protocol {
        Protocol::WSTP => match function.arguments_mode {
            ArgumentsMode::ExprList => gen_wstp::gen_arg_mode_expr_list(
                &function.item,
                function.name,
                wrapper_function_name.clone(),
            ),
            ArgumentsMode::PatternMatches {
                ref pattern,
                ref pattern_parameters,
            } => gen_wstp::gen_arg_mode_pattern(
                &function,
                wrapper_function_name.clone(),
                &pattern,
                &pattern_parameters,
            ),
        },
        Protocol::WXF => match function.arguments_mode {
            ArgumentsMode::ExprList => gen_wxf::gen_arg_mode_expr_list(
                &function.item,
                function.name,
                wrapper_function_name.clone(),
            ),
            ArgumentsMode::PatternMatches {
                ref pattern,
                ref pattern_parameters,
            } => gen_wxf::gen_arg_mode_pattern(
                &function,
                wrapper_function_name.clone(),
                &pattern,
                &pattern_parameters,
            ),
        },
    };

    // If the nightly Diagnostic API is available, use it to insert a `note:` which
    // includes the WL code needed to load the generated wrapper function. This hopefully
    // minimizes the effort needed on the part of the developer to discover how to load
    // their function.
    // TODO: Provide a mechanism for silencing these notes; when the user has more than a
    //       couple of `#[wolfram_library_function]` invocations, this quickly makes
    //       the build output quite noisy.
    // NOTE: One planned feature of wl-library-link is to provide safe wrappers around
    //       many of the other datatypes provided by C LibraryLink (e.g. SparseArray,
    //       Image, NumericArray's, etc.). This `note:` functionality will, I think, prove
    //       quite useful in those situations, because it will enable the user to specify
    //       the type of their function exactly using the safe wl-library-link wrapper
    //       types, and have the appropriate WL code to load the function generated
    //       automatically. The only thing the user will be required to do is copy and
    //       paste the changed WL whenever they make a change to the functions signature.
    #[cfg(feature = "nightly")]
    {
        use proc_macro::{Diagnostic, Level};

        // let location = match std::env::var("CARGO_MANIFEST_IR") {
        //     Ok(location) => format!("{}/target/debug/<name>.dylib", location),
        //     Err(_) => String::from("<path/to/library.dylib>"),
        // };
        let location = "_";

        let message = match protocol {
            Protocol::WSTP => format!(
                "load using `LibraryFunctionLoad[{location}, \"{wrapper}\", LinkObject, LinkObject]`",
                location = location,
                wrapper = wrapper_function_name,
            ),
            Protocol::WXF => format!(
                "load using `LibraryFunctionLoad[{location}, \"{wrapper}\", {{LibraryDataType[ByteArray]}}, LibraryDataType[ByteArray]]`",
                location = location,
                wrapper = wrapper_function_name,
            ),
        };

        Diagnostic::spanned(
            vec![function.item.sig.ident.span().unwrap()],
            Level::Note,
            message,
        )
        .emit()
    }

    Ok(tokens)
}

fn parse_attributes(attr_args: AttributeArgs) -> Result<Options> {
    use syn::NestedMeta;

    let mut name_option: Option<Ident> = None;
    let mut protocol: Option<Protocol> = None;

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
                    "expected `<name> = \"..\"` attribute`",
                ))
            },
        };

        if path.is_ident("name") {
            // Verify that we have not already parsed a value for the `name` option. E.g,
            // prevent `#[wolfram_library_function(name = "name1", name = "name2")]`.
            if name_option.is_some() {
                return Err(Error::new(path.span(), "attribute appears more than once"));
            }

            debug_assert!(name_option == None);
            name_option = Some(parse_name_option_value(lit)?);
        } else if path.is_ident("protocol") {
            if protocol.is_some() {
                return Err(Error::new(path.span(), "attribute appears more than once"));
            }

            debug_assert!(protocol == None);
            protocol = Some(parse_mode_option_value(lit)?);
        } else {
            return Err(Error::new(
                path.span(),
                "expected `name = \"..\"` or `protocol = \"..\"` attribute",
            ));
        }
    }

    Ok(Options {
        name: name_option,
        protocol,
    })
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

fn parse_mode_option_value(lit: syn::Lit) -> Result<Protocol> {
    let litstr = match lit {
        Lit::Str(string) => string,
        _ => return Err(Error::new(lit.span(), "expected string literal")),
    };

    let mode = match litstr.value().as_str() {
        "WXF" => Protocol::WXF,
        "WSTP" => Protocol::WSTP,
        _ => {
            return Err(Error::new(
                litstr.span(),
                "valid modes are 'WXF' and 'WSTP'",
            ))
        },
    };

    Ok(mode)
}
