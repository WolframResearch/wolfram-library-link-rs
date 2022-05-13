use wolfram_library_link::{
    self as wll, export,
    expr::{Expr, Symbol},
};

#[export(wstp)]
fn generate_message(_: Vec<Expr>) {
    // Construct the expression `Message[MySymbol::msg, "..."]`.
    let message = Expr::normal(Symbol::new("System`Message"), vec![
        // MySymbol::msg is MessageName[MySymbol, "msg"]
        Expr::normal(Symbol::new("System`MessageName"), vec![
            Expr::from(Symbol::new("Global`MySymbol")),
            Expr::string("msg"),
        ]),
        Expr::string("a Rust LibraryLink function"),
    ]);

    // Evaluate the message expression.
    let _: Expr = wll::evaluate(&message);
}
