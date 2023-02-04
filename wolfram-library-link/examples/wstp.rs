//! This example demonstrates how WSTP links can be used in LibraryLink functions to pass
//! arbitrary expressions as the function arguments and return value.

use wolfram_library_link::{
    self as wll,
    expr::{Expr, ExprKind, Number, Symbol},
    wstp::Link,
};

// Generates a special "loader" function, which returns an Association containing the
// loaded forms of all functions exported by this library.
//
// The loader can be loaded and used by evaluating:
//
// ```
// loadFunctions = LibraryFunctionLoad[
//     "libwstp_example",
//     "load_wstp_functions",
//     LinkObject,
//     LinkObject
// ];
//
// $functions = loadFunctions["libwstp_example"];
// ```
wll::generate_loader!(load_wstp_functions);

//======================================
// Using `&mut Link`
//======================================

//------------------
// square_wstp()
//------------------

/// Define a WSTP function that squares a number.
///
/// ```wolfram
/// square = $functions["square_wstp"];
///
/// square[4]    (* Returns 16 *)
/// ```
#[wll::export(wstp)]
fn square_wstp(link: &mut Link) {
    // Get the number of elements in the arguments list.
    let arg_count: usize = link.test_head("System`List").unwrap();

    if arg_count != 1 {
        panic!("square_wstp: expected to get a single argument");
    }

    // Get the argument value.
    let x = link.get_i64().expect("expected Integer argument");

    // Write the return value.
    link.put_i64(x * x).unwrap();
}

//------------------
// count_args()
//------------------

/// Define a function that returns an integer count of the number of arguments it was
/// given.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// countArgs = $functions["count_args"];
///
/// countArgs[a]          (* Returns 1)
/// countArgs[a, b, c]    (* Returns 3 *)
/// ```
#[wll::export(wstp)]
fn count_args(link: &mut Link) {
    // Get the number of elements in the arguments list.
    let arg_count: usize = link.test_head("System`List").unwrap();

    // Discard the remaining argument data.
    link.new_packet().unwrap();

    // Write the return value.
    link.put_i64(i64::try_from(arg_count).unwrap()).unwrap();
}

//------------------
// total_args_i64()
//------------------

/// Define a function that returns the sum of it's integer arguments.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// totalArgsI64 = $functions["total_args_i64"];
///
/// totalArgsI64[1, 1, 2, 3, 5]    (* Returns 12 *)
/// ```
#[wll::export(wstp)]
fn total_args_i64(link: &mut Link) {
    // Check that we recieved a functions arguments list, and get the number of arguments.
    let arg_count: usize = link.test_head("System`List").unwrap();

    let mut total: i64 = 0;

    // Get each argument, assuming that they are all integers, and add it to the total.
    for _ in 0..arg_count {
        let term = link.get_i64().expect("expected Integer argument");
        total += term;
    }

    // Write the return value to the link.
    link.put_i64(total).unwrap();
}

//------------------
// string_join()
//------------------

/// Define a function that will join its string arguments into a single string.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// stringJoin = $functions["string_join"];
///
/// stringJoin["Foo", "Bar"]           (* Returns "FooBar" *)
/// stringJoin["Foo", "Bar", "Baz"]    (* Returns "FooBarBaz" *)
/// stringJoin[]                       (* Returns "" *)
/// ```
#[wll::export(wstp)]
fn string_join(link: &mut Link) {
    use wstp::LinkStr;

    let arg_count = link.test_head("System`List").unwrap();

    let mut buffer = String::new();

    for _ in 0..arg_count {
        let elem: LinkStr<'_> = link.get_string_ref().expect("expected String argument");
        buffer.push_str(elem.as_str());
    }

    // Write the joined string value to the link.
    link.put_str(buffer.as_str()).unwrap();
}

//------------------
// link_expr_identity()
//------------------

/// Define a function that returns the argument expression that was sent over the link.
/// That expression will be a list of the arguments passed to this LibraryFunction[..].
///
/// ```wolfram
/// linkExprIdentity = $functions["link_expr_identity"];
///
/// linkExprIdentity[5]      (* Returns {5} *)
/// linkExprIdentity[a, b]   (* Returns {a, b} *)
/// ```
#[wll::export(wstp)]
fn link_expr_identity(link: &mut Link) {
    let expr = link.get_expr().unwrap();
    assert!(!link.is_ready());
    link.put_expr(&expr).unwrap();
}

//------------------
// expr_string_join()
//------------------

/// This example is an alternative to the `string_join()` example.
///
/// This example shows using the `Expr` and `ExprKind` types to process expressions on
/// the WSTP link.
#[wll::export(wstp)]
fn expr_string_join(link: &mut Link) {
    let expr = link.get_expr().unwrap();

    let list = expr.try_as_normal().unwrap();
    assert!(list.has_head(&Symbol::new("System`List")));

    let mut buffer = String::new();
    for elem in list.elements() {
        match elem.kind() {
            ExprKind::String(str) => buffer.push_str(str),
            _ => panic!("expected String argument, got: {:?}", elem),
        }
    }

    link.put_str(buffer.as_str()).unwrap()
}

//======================================
// Using `Vec<Expr>` argument list
//======================================

//------------------
// total()
//------------------

#[wll::export(wstp)]
fn total(args: Vec<Expr>) -> Expr {
    let mut total = Number::Integer(0);

    for (index, arg) in args.into_iter().enumerate() {
        let number = match arg.try_as_number() {
            Some(number) => number,
            None => panic!(
                "expected argument at position {} to be a number, got {}",
                // Add +1 to display using WL 1-based indexing.
                index + 1,
                arg
            ),
        };

        use Number::{Integer, Real};

        total = match (total, number) {
            // If the sum and new term are integers, use integers.
            (Integer(total), Integer(term)) => Integer(total + term),
            // Otherwise, if the either the total or new term are machine real numbers,
            // use floating point numbers.
            (Integer(int), Real(real)) | (Real(real), Integer(int)) => {
                Number::real(int as f64 + *real)
            },
            (Real(total), Real(term)) => Real(total + term),
        }
    }

    Expr::number(total)
}
