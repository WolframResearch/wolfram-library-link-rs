use wl_library_link::{wolfram_library_function, WolframEngine};

use wl_expr::Expr;

// #[test]
// fn main() {
//     // WRAPPED_ONE(5);
// }

#[wolfram_library_function]
pub fn wrapped_one(_: &WolframEngine, _: Vec<Expr>) -> Expr {
    Expr::string("success")
}

#[wolfram_library_function(protocol = "WXF")]
pub fn wxf_function(_: &WolframEngine, _: Vec<Expr>) -> Expr {
    Expr::string("success")
}
