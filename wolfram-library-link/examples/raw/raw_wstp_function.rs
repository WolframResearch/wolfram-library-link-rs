//! This example demonstrates using the raw Rust wrappers around the LibraryLink C API to
//! write a function which looks much like a classic C function using LibraryLink and
//! WSTP would.
//!
//! This also includes an example of mixing the low-level LibraryLink bindings with the
//! higher-level bindings provided by the `wstp` crate.

use std::os::raw::{c_int, c_uint};

use wolfram_library_link::{
    expr::{Expr, Symbol},
    sys::{
        self as wll_sys, WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
    },
    wstp::{
        sys::{WSGetInteger, WSNewPacket, WSPutInteger, WSTestHead, WSLINK},
        Link,
    },
};

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
    mut link: WSLINK,
) -> c_uint {
    // Create a safe Link wrapper around the raw `WSLINK`. This is a borrowed rather than
    // owned Link because the caller (the Kernel) owns the link.
    let link: &mut Link = unsafe { Link::unchecked_ref_cast_mut(&mut link) };

    // Skip reading the argument list packet.
    if link.raw_get_next().and_then(|_| link.new_packet()).is_err() {
        return LIBRARY_FUNCTION_ERROR;
    }

    let callback_link = unsafe { (*lib).getWSLINK.unwrap()(lib) };
    let mut callback_link = callback_link as wstp::sys::WSLINK;

    {
        let safe_callback_link =
            unsafe { Link::unchecked_ref_cast_mut(&mut callback_link) };

        safe_callback_link
            // EvaluatePacket[Print["Hello, World! --- WSTP"]]
            .put_expr(&Expr::normal(Symbol::new("System`EvaluatePacket"), vec![
                Expr::normal(Symbol::new("System`Print"), vec![Expr::string(
                    "Hello, World! --- WSTP",
                )]),
            ]))
            .unwrap();

        unsafe {
            (*lib).processWSLINK.unwrap()(
                safe_callback_link.raw_link() as wll_sys::WSLINK
            );
        }

        // Skip the return value packet. This is necessary, otherwise the link has
        // unread data and the return value of this function cannot be processed properly.
        if safe_callback_link
            .raw_get_next()
            .and_then(|_| safe_callback_link.new_packet())
            .is_err()
        {
            return LIBRARY_FUNCTION_ERROR;
        }
    }

    link.put_expr(&Expr::string("returned normally")).unwrap();

    return LIBRARY_NO_ERROR;
}

/// This example makes use of the [`wstp`][wstp] crate to provide a safe wrapper around
/// around the WSTP link object, which can be used to read the argument expression and
/// write out the return expression.
///
/// ```wolfram
/// function = LibraryFunctionLoad[
///     "raw_wstp_function",
///     "wstp_expr_function",
///     LinkObject,
///     LinkObject
/// ];
/// ```
#[no_mangle]
pub extern "C" fn wstp_expr_function(
    _lib: WolframLibraryData,
    mut unsafe_link: WSLINK,
) -> c_uint {
    let link: &mut Link = unsafe { Link::unchecked_ref_cast_mut(&mut unsafe_link) };

    let expr = match link.get_expr() {
        Ok(expr) => expr,
        Err(err) => {
            // Skip reading the argument list packet.
            if link.raw_get_next().and_then(|_| link.new_packet()).is_err() {
                return LIBRARY_FUNCTION_ERROR;
            }

            let err = Expr::string(err.to_string());
            let err = Expr::normal(Symbol::new("System`Failure"), vec![
                Expr::string("WSTP Error"),
                Expr::normal(Symbol::new("System`Association"), vec![Expr::normal(
                    Symbol::new("System`Rule"),
                    vec![Expr::string("Message"), err],
                )]),
            ]);
            match link.put_expr(&err) {
                Ok(()) => return LIBRARY_NO_ERROR,
                Err(_) => return LIBRARY_FUNCTION_ERROR,
            }
        },
    };

    let expr_string = format!("Input: {}", expr.to_string());

    match link.put_expr(&Expr::string(expr_string)) {
        Ok(()) => LIBRARY_NO_ERROR,
        Err(_) => LIBRARY_FUNCTION_ERROR,
    }
}
