use wl_expr::Expr;
use wolfram_library_link::{self as wll, wolfram_library_function, WolframEngine};

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
        wll::evaluate(&Expr! { Echo['arg] });
    }

    Expr::string(format!("finished echoing {} argument(s)", arg_count))
}
