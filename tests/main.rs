use wl_library_link::{generate_wrapper, Engine};

use wl_expr::Expr;

// #[test]
// fn main() {
//     // WRAPPED_ONE(5);
// }

generate_wrapper![WRAPPED_ONE # wrapped_one(a: Expr) -> Expr];
generate_wrapper![WRAPPED_TWO # wrapped_two(engine: Engine, a: Expr) -> Expr];

fn wrapped_one(a: Expr) -> Expr {
    a
}

fn wrapped_two(engine: Engine, a: Expr) -> Expr {
    a
}
