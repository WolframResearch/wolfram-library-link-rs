//! This example demonstrates how LibraryLink native data types can be used in Rust
//! functions called via LibraryLink.

use wolfram_library_link::{self as wll, NumericArray, UninitNumericArray};

//======================================
// Primitive data types
//======================================

//---------
// square()
//---------

/// Define a function to square a number.
///
/// The exported LibraryLink function may be loaded and used by evaluating:
///
/// ```wolfram
/// square = LibraryFunctionLoad["libbasic_types", "square", {Integer}, Integer];
/// square[2]
/// ```
fn square(x: i64) -> i64 {
    x * x
}

// Export the `square` function via LibraryLink. This will generate a "wrapper" function
// that correctly implements the lightweight LibraryLink <=> Rust conversion.
wll::export![square(_)];

//-----------------
// reverse_string()
//-----------------

fn reverse_string(string: String) -> String {
    string.chars().rev().collect()
}

wll::export![reverse_string(_)];

//------------------
// add2() and add3()
//------------------

fn add2(x: i64, y: i64) -> i64 {
    x + y
}

fn add3(x: i64, y: i64, z: i64) -> i64 {
    x + y + z
}

wll::export![
    add2(_, _);
    add3(_, _, _);
];

//======================================
// NumericArray's
//======================================

//------------
// total_i64()
//------------

// Load and use by evaluating:
//
// ```wolfram
// total = LibraryFunctionLoad[
//     "libbasic_types",
//     "total_i64",
//     {LibraryDataType[NumericArray, "Integer64"]},
//     Integer
// ];
//
// total[NumericArray[Range[100], "Integer64"]]
// ```
fn total_i64(list: &NumericArray<i64>) -> i64 {
    list.as_slice().into_iter().sum()
}

wll::export![total_i64(_)];

//---------------
// positive_i64()
//---------------

/// Get the sign of every element in `list` as a numeric array of 0's and 1's.
///
/// The returned array will have the same dimensions as `list`.
fn positive_i64(list: &NumericArray<i64>) -> NumericArray<u8> {
    let mut bools: UninitNumericArray<u8> =
        UninitNumericArray::from_dimensions(list.dimensions());

    for pair in list.as_slice().into_iter().zip(bools.as_slice_mut()) {
        let (elem, entry): (&i64, &mut std::mem::MaybeUninit<u8>) = pair;

        entry.write(u8::from(elem.is_positive()));
    }

    unsafe { bools.assume_init() }
}

wll::export![positive_i64(_)];

//======================================
// get_random_number()
//======================================

// Load and use by evaluating:
//
// ```wolfram
// randomNumber = LibraryFunctionLoad[
//     "libbasic_types",
//     "xkcd_get_random_number",
//     {},
//     Integer
// ];
// randomNumber[]
// ```
fn get_random_number() -> i64 {
    // chosen by fair dice roll.
    // guaranteed to be random.
    // xkcd.com/221
    4
}

wll::export![get_random_number() as xkcd_get_random_number];

//======================================
// raw_square()
//======================================

fn raw_square(args: &[wll::sys::MArgument], ret: wll::sys::MArgument) {
    if args.len() != 1 {
        panic!("unexpected number of arguments");
    }

    let x: i64 = unsafe { *args[0].integer };

    unsafe {
        *ret.integer = x * x;
    }
}

wll::export![raw_square(_, _)];
