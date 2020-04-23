//! This example demonstrates using the raw Rust wrappers around the LibraryLink C API to
//! write a function which looks much like a classic C function using LibraryLink would.

use std::os::raw::{c_int, c_uint};

use wl_library_link::sys::{
    WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
};
use wl_wstp::sys::{WSGetInteger, WSNewPacket, WSPutInteger, WSTestHead, WSLINK};

#[no_mangle]
pub unsafe extern "C" fn demo_wstp_function(
    _lib: WolframLibraryData,
    link: WSLINK,
) -> c_uint {
    let mut i1: c_int = 0;
    let mut i2: c_int = 0;
    let mut len: c_int = 0;

    if WSTestHead(link, b"List\0".as_ptr() as *const i8, &mut len) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if len != 2 {
        return LIBRARY_FUNCTION_ERROR;
    }

    if WSGetInteger(link, &mut i1) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if WSGetInteger(link, &mut i2) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if WSNewPacket(link) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    let sum = i1 + i2;

    if WSPutInteger(link, sum) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    return LIBRARY_NO_ERROR;
}
