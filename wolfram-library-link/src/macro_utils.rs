use std::{ffi::CString, os::raw::c_uint};

use wl_expr::{forms::ToPrettyExpr, Expr, ExprKind};
use wstp::{self, Link};

use crate::{
    catch_panic,
    sys::{self, MArgument, LIBRARY_NO_ERROR},
    NativeFunction, NumericArray,
};

//======================================
// #[wolfram_library_function] helpers
//======================================

//==================
// WSTP helpers
//==================

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wstp_wolfram_library_function_expr_list(
    libdata: sys::WolframLibraryData,
    unsafe_link: wstp::sys::WSLINK,
    function: fn(Vec<Expr>) -> Expr,
) -> c_uint {
    call_wstp_wolfram_library_function(
        libdata,
        unsafe_link,
        |argument_expr: Expr| -> Expr {
            let arguments = match argument_expr.to_kind() {
                ExprKind::Normal(normal) => normal.contents,
                _ => panic!("WSTP argument expression was non-Normal"),
            };

            function(arguments)
        },
    )
}

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wstp_wolfram_library_function<
    F: FnOnce(Expr) -> Expr + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    mut unsafe_link: wstp::sys::WSLINK,
    function: F,
) -> c_uint {
    use self::{
        catch_panic::{call_and_catch_panic, CaughtPanic},
        wstp::sys::{WSEndPacket, WSPutString},
    };

    let _ = crate::initialize(libdata);

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(move || {
            let link = Link::unchecked_ref_cast_mut(&mut unsafe_link);

            let arguments: Expr = match link.get_expr() {
                Ok(args) => args,
                Err(message) => {
                    // Skip reading the argument list packet.
                    if link.raw_get_next().and_then(|_| link.new_packet()).is_err() {
                        return;
                    }

                    let _: Result<_, _> = link.put_expr(&Expr! {
                        Failure["LibraryFunctionWSTPError", <|
                            "Message" -> %[Expr::string(message.to_string())]
                        |>]
                    });
                    return;
                },
            };

            let result: Expr = function(arguments);

            link.put_expr(&result).expect(
                "LibraryFunction result expression could not be written to WSTP link",
            );
        })
    };

    match result {
        Ok(()) => LIBRARY_NO_ERROR,
        Err(caught_panic) => unsafe {
            // FIXME: Fix unwraps + return this as a full expr
            let cstring =
                CString::new(caught_panic.to_pretty_expr().to_string()).unwrap();

            WSPutString(unsafe_link, cstring.as_ptr());

            WSEndPacket(unsafe_link);

            LIBRARY_NO_ERROR
        },
    }
}

//==================
// WXF helpers
//==================

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wxf_wolfram_library_function_expr_list(
    libdata: sys::WolframLibraryData,
    wxf_argument: MArgument,
    wxf_result: MArgument,
    function: fn(Vec<Expr>) -> Expr,
) -> c_uint {
    call_wxf_wolfram_library_function(
        libdata,
        wxf_argument,
        wxf_result,
        |argument_expr: Expr| -> Expr {
            let arguments = match argument_expr.to_kind() {
                ExprKind::Normal(normal) => normal.contents,
                _ => panic!("WXF argument expression was non-Normal"),
            };

            function(arguments)
        },
    )
}

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wxf_wolfram_library_function<
    F: FnOnce(Expr) -> Expr + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    wxf_argument: MArgument,
    wxf_result: MArgument,
    function: F,
) -> c_uint {
    use self::catch_panic::{call_and_catch_panic, CaughtPanic};

    let _ = crate::initialize(libdata);

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(|| {
            let argument_numeric_array = NumericArray::from_raw(*wxf_argument.numeric)
                .try_into_kind::<u8>()
                .expect(
                    "wolfram_library_function: expected NumericArray of UnsignedInteger8",
                );

            let arguments = wxf::deserialize(argument_numeric_array.as_slice()).expect(
                "wolfram_library_function: failed to deserialize argument WXF data",
            );

            let result: Expr = function(arguments);

            *wxf_result.numeric = wxf_numeric_array_from_expr(&result).into_raw();
        })
    };

    match result {
        Ok(()) => LIBRARY_NO_ERROR,
        // NOTE: This block tries to minimize calls to functions which could potentially
        //       panic, on a best-effort basis. If a panic were to occur within this code
        //       it would not be caught and the Rust stack unwinder would likely abort
        //       the Kernel process, which isn't very user friendly.
        Err(caught_panic) => {
            let pretty_expr = caught_panic.to_pretty_expr();

            unsafe {
                *wxf_result.numeric =
                    wxf_numeric_array_from_expr(&pretty_expr).into_raw();
            }

            LIBRARY_NO_ERROR
        },
    }
}

fn wxf_numeric_array_from_expr(expr: &Expr) -> NumericArray<u8> {
    let result_wxf: Vec<u8> = wxf::serialize(expr)
        .expect("wolfram_library_function: failed to serialize result expression to WXF");

    NumericArray::from_slice(result_wxf.as_slice())
}

//======================================
// NativeFunction helpers
//======================================

pub unsafe fn call_native_wolfram_library_function<'a, F: NativeFunction<'a>>(
    lib_data: sys::WolframLibraryData,
    args: *mut MArgument,
    argc: sys::mint,
    res: MArgument,
    func: F,
) -> c_uint {
    use std::panic::{self, AssertUnwindSafe};

    // Initialize the library.
    if crate::initialize(lib_data).is_err() {
        return sys::LIBRARY_FUNCTION_ERROR;
    }

    let argc = match usize::try_from(argc) {
        Ok(argc) => argc,
        Err(_) => return sys::LIBRARY_FUNCTION_ERROR,
    };

    let args: &[MArgument] = std::slice::from_raw_parts(args, argc);

    if panic::catch_unwind(AssertUnwindSafe(move || func.call(args, res))).is_err() {
        // TODO: Store the panic into a "LAST_ERROR" static, and provide an accessor to
        //       get it from WL? E.g. RustLink`GetLastError[<optional func name>].
        return sys::LIBRARY_FUNCTION_ERROR;
    };

    sys::LIBRARY_NO_ERROR
}
