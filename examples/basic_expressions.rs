use wl_expr::Expr;
use wl_expr_macro::Expr;
use wl_lang::forms::ToExpr;
use wl_library_link::{wolfram_library_function, WolframEngine};

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libbasic_expressions.dylib",
///     "echo_arguments_wrapper",
///     LinkObject,
///     LinkObject
/// ]
/// ```
#[wolfram_library_function]
pub fn echo_arguments(engine: &WolframEngine, args: Vec<Expr>) -> Expr {
    let arg_count = args.len();

    for arg in args {
        engine.evaluate(&Expr! { Echo['arg] });
    }

    Expr::string(format!("finished echoing {} argument(s)", arg_count))
}
