use wl_expr::{forms::Sequence, Expr, Number};
use wolfram_library_link::wolfram_library_function;

#[wolfram_library_function]
#[pattern(arg1_?NumberQ, arg2_String, rest___)]
pub fn downvalue_example(arg1: Number, arg2: String, rest: Sequence<Expr>) -> Expr {
    Expr! {
        "Arguments"['arg1, 'arg2, 'rest]
    }
}
