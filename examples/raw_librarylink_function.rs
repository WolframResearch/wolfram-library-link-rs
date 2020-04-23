//! This example demonstrates using the raw Rust wrappers around the LibraryLink C API to
//! write a function which looks much like a classic C function using LibraryLink would.

use std::os::raw::c_uint;

use wl_library_link::sys::{
    mint, MArgument, WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
};

#[no_mangle]
pub unsafe extern "C" fn demo_function(
    _lib_data: WolframLibraryData,
    arg_count: mint,
    args: *mut MArgument,
    res: MArgument,
) -> c_uint {
    if arg_count != 2 {
        return LIBRARY_FUNCTION_ERROR;
    }

    let a: i64 = *(*args.offset(0)).integer;
    let b: i64 = *(*args.offset(1)).integer;

    *res.integer = a + b;

    LIBRARY_NO_ERROR
}
