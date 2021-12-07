use std::ffi::CString;

use wolfram_library_link::{
    self as wll,
    sys::{mint, mreal},
    NumericArray, UninitNumericArray,
};

//======================================
// Primitive data types
//======================================

wll::export![
    test_no_args();
    test_ret_void();
    test_mint(_);
    test_raw_mint(_, _);
    test_mint_mint(_, _);
    test_mreal(_);
    test_i64(_);
    test_i64_i64(_, _);
    test_f64(_);
    // test_str(_);
    test_string(_);
    test_c_string(_);
];

fn test_no_args() -> i64 {
    4
}

fn test_ret_void() {
    // Do nothing.
}

//------------
// mint, mreal
//------------

fn test_mint(x: mint) -> mint {
    x * x
}

// Test NativeFunction impl for raw function using raw MArguments.
fn test_raw_mint(args: &[wll::sys::MArgument], ret: wll::sys::MArgument) {
    if args.len() != 1 {
        panic!("unexpected number of arguments");
    }

    let x: mint = unsafe { *args[0].integer };

    unsafe {
        *ret.integer = x * x;
    }
}

fn test_mint_mint(x: mint, y: mint) -> mint {
    x + y
}

fn test_mreal(x: mreal) -> mreal {
    x * x
}

//------------
// i64, f64
//------------

fn test_i64(x: i64) -> i64 {
    x * x
}

fn test_i64_i64(x: i64, y: i64) -> i64 {
    x + y
}

fn test_f64(x: f64) -> f64 {
    x * x
}

//--------
// Strings
//--------

// fn test_str(string: &str) -> String {
//     string.chars().rev().collect()
// }

fn test_string(string: String) -> String {
    string.chars().rev().collect()
}

fn test_c_string(string: CString) -> i64 {
    i64::try_from(string.as_bytes().len()).expect("string len usize overflows i64")
}

//======================================
// NumericArray's
//======================================

wll::export![
    total_i64(_);
    positive_i64(_);
];

fn total_i64(list: &NumericArray<i64>) -> i64 {
    list.as_slice().into_iter().sum()
}

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
