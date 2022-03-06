use std::ffi::CString;

use wolfram_library_link::{
    self as wll,
    sys::{mint, mreal},
    NumericArray, UninitNumericArray,
};

//======================================
// Primitive data types
//======================================

#[wll::export]
fn test_no_args() -> i64 {
    4
}

#[wll::export]
fn test_ret_void() {
    // Do nothing.
}

//------------
// mint, mreal
//------------

#[wll::export]
fn test_mint(x: mint) -> mint {
    x * x
}

// Test NativeFunction impl for raw function using raw MArguments.
#[wll::export]
fn test_raw_mint(args: &[wll::sys::MArgument], ret: wll::sys::MArgument) {
    if args.len() != 1 {
        panic!("unexpected number of arguments");
    }

    let x: mint = unsafe { *args[0].integer };

    unsafe {
        *ret.integer = x * x;
    }
}

#[wll::export]
fn test_mint_mint(x: mint, y: mint) -> mint {
    x + y
}

#[wll::export]
fn test_mreal(x: mreal) -> mreal {
    x * x
}

//------------
// i64, f64
//------------

#[wll::export]
fn test_i64(x: i64) -> i64 {
    x * x
}

#[wll::export]
fn test_i64_i64(x: i64, y: i64) -> i64 {
    x + y
}

#[wll::export]
fn test_f64(x: f64) -> f64 {
    x * x
}

//--------
// Strings
//--------

// fn test_str(string: &str) -> String {
//     string.chars().rev().collect()
// }

#[wll::export]
fn test_string(string: String) -> String {
    string.chars().rev().collect()
}

#[wll::export]
fn test_c_string(string: CString) -> i64 {
    i64::try_from(string.as_bytes().len()).expect("string len usize overflows i64")
}

//-------
// Panics
//-------

#[wll::export]
fn test_panic() {
    panic!("this function panicked");
}

//======================================
// NumericArray's
//======================================

#[wll::export]
fn total_i64(list: &NumericArray<i64>) -> i64 {
    list.as_slice().into_iter().sum()
}

/// Get the sign of every element in `list` as a numeric array of 0's and 1's.
///
/// The returned array will have the same dimensions as `list`.
#[wll::export]
fn positive_i64(list: &NumericArray<i64>) -> NumericArray<u8> {
    let mut bools: UninitNumericArray<u8> =
        UninitNumericArray::from_dimensions(list.dimensions());

    for pair in list.as_slice().into_iter().zip(bools.as_slice_mut()) {
        let (elem, entry): (&i64, &mut std::mem::MaybeUninit<u8>) = pair;

        entry.write(u8::from(elem.is_positive()));
    }

    unsafe { bools.assume_init() }
}
