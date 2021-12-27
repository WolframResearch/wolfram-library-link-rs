//! This example demonstrates how WSTP links can be used in LibraryLink functions to pass
//! arbitrary expressions as the function arguments and return value.

use wolfram_library_link::{self as wll, wstp::Link};

//------------------
// square_wstp()
//------------------

wll::export_wstp![square_wstp];

/// Define a WSTP function that squares a number.
///
/// ```wolfram
/// square = LibraryFunctionLoad[
///     "libwstp_example",
///     "square_wstp",
///     LinkObject,
///     LinkObject
/// ];
///
/// square[4]    (* Returns 16 *)
/// ```
fn square_wstp(link: &mut Link) {
    // Get the number of elements in the arguments list.
    let arg_count: usize = link.test_head("List").unwrap();

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

wll::export_wstp![count_args];

/// Define a function that returns an integer count of the number of arguments it was
/// given.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// countArgs = LibraryFunctionLoad[
///     "libwstp_example",
///     "count_args",
///     LinkObject,
///     LinkObject
/// ]
///
/// countArgs[a]          (* Returns 1)
/// countArgs[a, b, c]    (* Returns 3 *)
/// ```
fn count_args(link: &mut Link) {
    // Get the number of elements in the arguments list.
    let arg_count: usize = link.test_head("List").unwrap();

    // Discard the remaining argument data.
    link.new_packet().unwrap();

    // Write the return value.
    link.put_i64(i64::try_from(arg_count).unwrap()).unwrap();
}

//------------------
// total_args_i64()
//------------------

wll::export_wstp![total_args_i64];

/// Define a function that returns the sum of it's integer arguments.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// totalArgsI64 = LibraryFunctionLoad[
///     "libwstp_example",
///     "total_args_i64",
///     LinkObject,
///     LinkObject
/// ];
///
/// totalArgsI64[1, 1, 2, 3, 5]    (* Returns 12 *)
/// ```
fn total_args_i64(link: &mut Link) {
    // Check that we recieved a functions arguments list, and get the number of arguments.
    let arg_count: usize = link.test_head("List").unwrap();

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

wll::export_wstp![string_join];

/// Define a function that will join its string arguments into a single string.
///
/// The exported LibraryLink function can be loaded and used by evaluating:
///
/// ```wolfram
/// stringJoin = LibraryFunctionLoad[
///     "libwstp_example",
///     "string_join",
///     LinkObject,
///     LinkObject
/// ];
///
/// stringJoin["Foo", "Bar"]           (* Returns "FooBar" *)
/// stringJoin["Foo", "Bar", "Baz"]    (* Returns "FooBarBaz" *)
/// stringJoin[]                       (* Returns "" *)
/// ```
fn string_join(link: &mut Link) {
    use wstp::LinkStr;

    let arg_count = link.test_head("List").unwrap();

    let mut buffer = String::new();

    for _ in 0..arg_count {
        let elem: LinkStr<'_> = link.get_string_ref().expect("expected String argument");
        buffer.push_str(elem.to_str());
    }

    // Write the joined string value to the link.
    link.put_str(buffer.as_str()).unwrap();
}

//------------------
// link_expr_identity()
//------------------

wll::export_wstp!(link_expr_identity);

/// Define a function that returns the argument expression that was sent over the link.
/// That expression will be a list of the arguments passed to this LibraryFunction[..].
///
/// ```wolfram
/// linkExprIdentity = LibraryFunctionLoad[
///     "libwstp_example",
///     "link_expr_identity",
///     LinkObject,
///     LinkObject
/// ];
///
/// Block[{$Context = "UnusedContext`", $ContextPath = {}},
///     linkExprIdentity[5]      (* Returns {5} *)
///     linkExprIdentity[a, b]   (* Returns {a, b} *)
/// ]
/// ```
fn link_expr_identity(link: &mut Link) {
    let expr = link.get_expr().unwrap();
    assert!(!link.is_ready());
    link.put_expr(&expr).unwrap();
}
