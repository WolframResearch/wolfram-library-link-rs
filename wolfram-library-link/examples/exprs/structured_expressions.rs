//! TODO: This example isn't the greatest -- come up with another example and use it.

use wl_expr::{
    forms::{FormError, FromExpr, List},
    Expr, Number,
};
use wl_pattern_match::FromExpr;
use wolfram_library_link::{self as wll, wolfram_library_function};

#[derive(FromExpr)]
#[pattern(numbers:{___})]
struct Numbers {
    numbers: List<Number>,
}

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libstructured_expressions.dylib",
///     "sum_of_numbers_wrapper",
///     LinkObject,
///     LinkObject
/// ]
/// ```
#[wolfram_library_function]
pub fn sum_of_numbers(arguments: Vec<Expr>) -> Expr {
    let Numbers { numbers } = match Numbers::from_expr(&arguments[0]) {
        Ok(numbers) => numbers,
        Err(err) => {
            return Expr! {
                Failure["ArgumentShape", <|
                    "Message" -> %[format!("{}", FormError::from(err))]
                |>]
            }
        },
    };

    let List(numbers) = numbers;

    let mut sum: f64 = 0.0;

    for number in numbers {
        wll::evaluate(&Expr! { Echo['number] });

        match number {
            Number::Integer(int) => sum += int as f64,
            Number::Real(real) => sum += *real,
        }
    }

    Expr::number(Number::Real(wl_expr::F64::new(sum).expect("got NaN")))
}
