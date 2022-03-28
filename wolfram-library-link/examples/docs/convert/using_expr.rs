use wolfram_library_link::{
    export,
    expr::{Expr, Symbol},
};

struct Point {
    x: f64,
    y: f64,
}

#[export(wstp)]
fn create_point2(args: Vec<Expr>) -> Expr {
    assert!(args.is_empty());

    let point = Point { x: 3.0, y: 4.0 };

    point.to_expr()
}

impl Point {
    fn to_expr(&self) -> Expr {
        let Point { x, y } = *self;

        Expr::normal(Symbol::new("System`Point"), vec![Expr::list(vec![
            Expr::real(x),
            Expr::real(y),
        ])])
    }
}
