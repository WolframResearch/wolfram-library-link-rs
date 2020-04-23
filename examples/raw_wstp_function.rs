//! This example demonstrates using the raw Rust wrappers around the LibraryLink C API to
//! write a function which looks much like a classic C function using LibraryLink would.

use std::os::raw::{c_int, c_uint};

use wl_library_link::sys::{
    WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
};
use wl_wstp::sys::{WSGetInteger, WSNewPacket, WSPutInteger, WSTestHead, WSLINK};

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libraw_wstp_function.dylib",
///     "demo_wstp_function",
///     LinkObject,
///     LinkObject
/// ]
/// ```
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

/// This example shows how the raw Rust wrappers can be mixed with higher-level wrappers
/// around the Wolfram Symbolic Transfer Protocal (WSTP) for conveniently calling back
/// into the Kernel to perform an evaluation.
///
/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libraw_wstp_function.dylib",
///     "demo_wstp_function_callback",
///     LinkObject,
///     LinkObject
/// ]
/// ```
#[no_mangle]
pub extern "C" fn demo_wstp_function_callback(
    lib: WolframLibraryData,
    link: WSLINK,
) -> c_uint {
    let link = unsafe { WSTPLink::new(link) };

    link.get_expr().unwrap();

    let callback_link = unsafe { (*lib).getWSLINK.unwrap()(lib) };

    {
        let safe_callback_link = unsafe { WSTPLink::new(callback_link as *mut _) };

        safe_callback_link
            .put_expr(&Expr! {
                EvaluatePacket[Print["Hello, World! --- WSTP"]]
            })
            .unwrap();

        unsafe {
            (*lib).processWSLINK.unwrap()(callback_link);
        }

        // FIXME: Is this necessary? -- Answer, yes, otherwise the link has unread data.
        if let Err(message) = safe_callback_link.get_expr() {
            link.put_expr(&Expr::string(message)).unwrap();
            return LIBRARY_NO_ERROR;
        }
    }

    link.put_expr(&Expr::string("returned normally")).unwrap();

    return LIBRARY_NO_ERROR;
}
