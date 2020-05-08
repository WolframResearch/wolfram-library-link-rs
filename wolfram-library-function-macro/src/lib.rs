extern crate proc_macro;

mod function;
mod gen_wstp;

use proc_macro2::TokenStream;

use syn::{
    spanned::Spanned, AttributeArgs, Error, Ident, Lit,
    Meta, MetaNameValue, Result,
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

    let tokens = match function.arguments_mode {
        ArgumentsMode::ExprList => {
            gen_wstp::gen_arg_mode_expr_list(&function.item, function.name, wrapper_function_name)
        },
        ArgumentsMode::PatternMatches {
            ref pattern,
            ref pattern_parameters,
        } => gen_wstp::gen_arg_mode_pattern(
            &function,
            wrapper_function_name,
            &pattern,
            &pattern_parameters,
        ),
    };

    Ok(tokens)
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


