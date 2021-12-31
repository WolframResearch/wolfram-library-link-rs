use wl_expr::Expr;
use wolfram_library_link::{self as wll};

wll::export_wstp![echo_arguments(_)];

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libbasic_expressions.dylib",
///     "echo_arguments",
///     LinkObject,
///     LinkObject
/// ]
/// ```
pub fn echo_arguments(args: Vec<Expr>) -> Expr {
    let arg_count = args.len();

    for arg in args {
        wll::evaluate(&Expr! { Echo['arg] });
    }

    Expr::string(format!("finished echoing {} argument(s)", arg_count))
}
