use std::ffi::CString;

use wl_expr::{forms::ToPrettyExpr, Expr, ExprKind};
use wstp::{self, Link};

use crate::{
    catch_panic,
    sys::{self, MArgument, LIBRARY_NO_ERROR},
    NumericArray, WolframEngine,
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
    function: fn(&WolframEngine, Vec<Expr>) -> Expr,
) -> std::os::raw::c_uint {
    call_wstp_wolfram_library_function(
        libdata,
        unsafe_link,
        |engine: &WolframEngine, argument_expr: Expr| -> Expr {
            let arguments = match argument_expr.to_kind() {
                ExprKind::Normal(normal) => normal.contents,
                _ => panic!("WSTP argument expression was non-Normal"),
            };

            function(engine, arguments)
        },
    )
}

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wstp_wolfram_library_function<
    F: FnOnce(&WolframEngine, Expr) -> Expr + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    mut unsafe_link: wstp::sys::WSLINK,
    function: F,
) -> std::os::raw::c_uint {
    use self::{
        catch_panic::{call_and_catch_panic, CaughtPanic},
        wstp::sys::{WSEndPacket, WSPutString},
    };

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(move || {
            // Contruct the engine
            let engine = WolframEngine::from_library_data(libdata);

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

            let result: Expr = function(&engine, arguments);

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
    function: fn(&WolframEngine, Vec<Expr>) -> Expr,
) -> std::os::raw::c_uint {
    call_wxf_wolfram_library_function(
        libdata,
        wxf_argument,
        wxf_result,
        |engine: &WolframEngine, argument_expr: Expr| -> Expr {
            let arguments = match argument_expr.to_kind() {
                ExprKind::Normal(normal) => normal.contents,
                _ => panic!("WXF argument expression was non-Normal"),
            };

            function(engine, arguments)
        },
    )
}

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wxf_wolfram_library_function<
    F: FnOnce(&WolframEngine, Expr) -> Expr + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    wxf_argument: MArgument,
    wxf_result: MArgument,
    function: F,
) -> std::os::raw::c_uint {
    use self::catch_panic::{call_and_catch_panic, CaughtPanic};

    let _ = crate::initialize(libdata);

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(|| {
            // Contruct the engine
            let engine = WolframEngine::from_library_data(libdata);

            let argument_numeric_array = NumericArray::from_raw(*wxf_argument.numeric)
                .try_into_kind::<u8>()
                .expect(
                    "wolfram_library_function: expected NumericArray of UnsignedInteger8",
                );

            let arguments = wxf::deserialize(argument_numeric_array.as_slice()).expect(
                "wolfram_library_function: failed to deserialize argument WXF data",
            );

            let result: Expr = function(&engine, arguments);

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
        .expect("wolfram_library_function: failed to construct NumericArray<u8>")
}
