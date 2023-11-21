use wolfram_library_link::{
    export,
    kernel::{Expr, NormalExpr, SymbolExpr},
};

#[export]
fn test_kernel_expr_create_string() {
    let list = NormalExpr::list_from_array([
        Expr::mint(1),
        Expr::string("two"),
        Expr::mreal(3.5),
    ]);

    // $ReturnValue = list
    SymbolExpr::lookup("Global`$ReturnValue").set_to(&list.as_expr());
}

#[export]
fn test_kernel_expr_create_symbols() {
    let list = NormalExpr::list_from_array([
        SymbolExpr::lookup("Example1").into(),
        SymbolExpr::lookup("`Example2").into(),
        SymbolExpr::lookup("Example3`Example4").into(),
    ]);

    // $ReturnValue = list
    SymbolExpr::lookup("Global`$ReturnValue").set_to(&list.as_expr());
}

#[export]
fn test_kernel_expr_create_heterogenous() {
    let result = NormalExpr::list_from_array([
        Expr::mint(1),
        Expr::mreal(2.01),
        Expr::string("three"),
        Expr::symbol("Global`Four"),
        Expr::list_from_array([Expr::string("a"), Expr::string("b"), Expr::string("c")]),
    ]);

    // $ReturnValue = list
    SymbolExpr::lookup("Global`$ReturnValue").set_to(&result.as_expr());
}
