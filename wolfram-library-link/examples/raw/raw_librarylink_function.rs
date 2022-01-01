//! This example demonstrates using the raw Rust wrappers around the LibraryLink C API to
//! write a function which looks much like a classic C function using LibraryLink would.

use std::mem::MaybeUninit;
use std::os::raw::c_uint;

use wolfram_library_link::sys::{
    mint, MArgument, MNumericArray, MNumericArray_Data_Type, WolframLibraryData,
    LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
};

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/target/debug/examples/libraw_librarylink_function.dylib",
///     "demo_function",
///     {Integer, Integer},
///     Integer
/// ]
/// ```
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

//======================================
// NumericArray's
//======================================

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/target/debug/examples/libraw_librarylink_function.dylib",
///     "demo_function",
///     {},
///     LibraryDataType[ByteArray]
/// ]
/// ```
#[no_mangle]
pub unsafe extern "C" fn demo_byte_array(
    lib_data: WolframLibraryData,
    arg_count: mint,
    _args: *mut MArgument,
    res: MArgument,
) -> c_uint {
    const LENGTH: usize = 10;

    if arg_count != 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    let na_funs = *(*lib_data).numericarrayLibraryFunctions;

    //
    // Allocate a new MNumericArray with 10 u8 elements
    //

    let mut byte_array: MNumericArray = std::ptr::null_mut();

    let err = (na_funs.MNumericArray_new.unwrap())(
        MNumericArray_Data_Type::MNumericArray_Type_UBit8,
        1,
        &10,
        &mut byte_array,
    );
    if err != 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    //
    // Fill the NumericArray with the number 1 to 10
    //

    let data_ptr: *mut std::ffi::c_void =
        (na_funs.MNumericArray_getData.unwrap())(byte_array);
    let data_ptr = data_ptr as *mut MaybeUninit<u8>;

    let slice = std::slice::from_raw_parts_mut(data_ptr, LENGTH);

    for (index, elem) in slice.iter_mut().enumerate() {
        *elem = MaybeUninit::new(index as u8)
    }

    //
    // Return the NumericArray
    //

    *res.numeric = byte_array;
    LIBRARY_NO_ERROR
}
