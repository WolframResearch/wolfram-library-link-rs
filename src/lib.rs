//! A safe and convenient wrapper around wl-library-link-sys.
//!
//! # Automatically generating LibraryLink wrappers around Rust functions.
//!
//! See the [`generate_wrapper!()`][macro@generate_wrapper] macro.
//!
//! ## Show backtrace when a panic occurs
//!
//! `generate_wrapper!()` will automatically catch any Rust panic's which occur in the
//! wrapped code, and show an error in the FE with the panic message and source file/line
//! number. It also can optionally show the backtrace. This is configured by the
//! "LIBRARY_LINK_RUST_BACKTRACE" environment variable. Enable it by evaluating:
//!
//! ```wolfram
//! SetEnvironment["LIBRARY_LINK_RUST_BACKTRACE" -> "True"]
//! ```
//!
//! Now the error shown when a panic occurs will include a backtrace.

#![cfg_attr(feature = "nightly", feature(panic_info_message))]

pub mod catch_panic;

use std::ffi::CString;

use wl_expr::{Expr, ExprKind, Symbol};
use wl_expr_macro::Expr;
use wl_lang::forms::ToExpr;
use wl_library_link_sys::{mint, WolframLibraryData, LIBRARY_NO_ERROR, MLINK};
use wl_symbol_table as sym;
use wl_wstp::WSTPLink;

// Re-export `wl_library_link_sys` and `wl_wstp`.
//
// TODO(!): Only selectively re-export the parts of these API's which are actually
//          needed? These should at least have module documentation saying that they
//          shouldn't be used?

/// Re-export of `wl_library_link_sys`
pub use wl_library_link_sys as sys;
pub use wl_wstp as wstp;

pub use wolfram_library_function_macro::wolfram_library_function;

const BACKTRACE_ENV_VAR: &str = "LIBRARY_LINK_RUST_BACKTRACE";

//======================================
// WolframEngine
//======================================

/// This struct should be considered private.
///
/// It is only public because it appears in the expansion of `generate_wrapper`.
#[allow(non_snake_case)]
pub struct WolframEngine {
    wl_lib: WolframLibraryData,

    // TODO: Is this function thread safe? Can it be called from a thread other than the
    //       one the LibraryLink wrapper was originally invoked from?
    AbortQ: unsafe extern "C" fn() -> mint,
    getWSLINK: unsafe extern "C" fn(WolframLibraryData) -> MLINK,
    processWSLINK: unsafe extern "C" fn(MLINK) -> i32,
}

impl From<WolframLibraryData> for WolframEngine {
    fn from(libdata: WolframLibraryData) -> Self {
        // TODO(!): Use the library version to verify this is still correct?
        // TODO(!): Audit this
        // NOTE: That these fields are even an Option is likely just bindgen being
        //       conservative with function pointers possibly being null.
        // TODO: Investigate making bindgen treat these as non-null fields?
        unsafe {
            let lib = *libdata;
            WolframEngine {
                wl_lib: libdata,

                AbortQ: lib.AbortQ.expect("AbortQ callback is NULL"),
                getWSLINK: lib.getWSLINK.expect("getWSLINK callback is NULL"),
                processWSLINK: lib.processWSLINK.expect("processWSLINK callback is NULL"),
            }
        }
    }
}

impl WolframEngine {
    /// Returns `true` if the user has requested that the current evaluation be aborted.
    ///
    /// Programs should finish what they are doing and return control of this thread to
    /// to the kernel as quickly as possible. They should not exit the process or
    /// otherwise terminate execution, simply return up the call stack.
    pub fn aborted(&self) -> bool {
        let val: mint = unsafe { (self.AbortQ)() };
        val == 1
    }

    pub fn evaluate(&self, expr: &Expr) -> Expr {
        unsafe {
            let unsafe_link = (self.getWSLINK)(self.wl_lib);
            // Go from *mut MLINK -> *mut WSLINK
            let link = WSTPLink::new(unsafe_link as *mut _);

            // Send an EvaluatePacket['expr].
            let _: () = link
                .put_expr(&Expr! { EvaluatePacket['expr] })
                .expect("WolframEngine::evaluate: failed to send EvaluatePacket");

            // Process the packet on the link.
            let code: i32 = (self.processWSLINK)(unsafe_link);

            if code == 0 {
                // TODO: Use WSErrorMessage() here and print the error.
                panic!("WolframEngine::evaluate: processWSLINKK returned error",);
            }

            let return_packet: Expr = link.get_expr().expect(
                "WolframEngine::evaluate: failed to read return packet from WSTP link",
            );

            let returned_expr = match return_packet.kind() {
                ExprKind::Normal(normal) => {
                    debug_assert!(normal.has_head(&*sym::ReturnPacket));
                    debug_assert!(normal.contents.len() == 1);
                    normal.contents[0].clone()
                },
                _ => panic!(
                    "WolframEngine::evaluate: returned expression was not ReturnPacket: {}",
                    return_packet
                ),
            };

            returned_expr
        }
    }

    fn try_evaluate(&self, expr: &Expr) -> Result<Expr, String> {
        let link = self.get_wstp_link();

        // Send an EvaluatePacket['expr].
        let _: () = link.put_expr(&Expr! { EvaluatePacket['expr] })?;

        let _: () = self.process_wstp_link(&link)?;

        let return_packet: Expr = link.get_expr()?;

        let returned_expr = match return_packet.kind() {
            ExprKind::Normal(normal) => {
                debug_assert!(normal.has_head(&*sym::ReturnPacket));
                debug_assert!(normal.contents.len() == 1);
                normal.contents[0].clone()
            },
            _ => panic!(
                "WolframEngine::evaluate: returned expression was not ReturnPacket: {}",
                return_packet
            ),
        };

        Ok(returned_expr)
    }

    // PRE-COMMIT: SHould be private?
    pub fn get_wstp_link(&self) -> WSTPLink {
        unsafe {
            let unsafe_link = (self.getWSLINK)(self.wl_lib);
            // Go from *mut MLINK -> *mut WSLINK
            WSTPLink::new(unsafe_link as *mut _)
        }
    }

    fn process_wstp_link(&self, link: &WSTPLink) -> Result<(), String> {
        let raw_link = unsafe { link.raw_link() };

        // Process the packet on the link.
        let code: i32 = unsafe { (self.processWSLINK)(raw_link as *mut _) };

        if code == 0 {
            let error_message = link
                .error_message()
                .unwrap_or_else(|| "unknown error occurred on WSTPLink".into());

            return Err(error_message);
        }

        Ok(())
    }

    // TODO?: Add a try_evaluate() -> Result<Expr, String> method, to more gracefully
    //        handle situations where the created WSLINK can fail? Do we anticipate that
    //        that could ever happen in a reasonable situation, where the error could be
    //        handled reasonably?

    // TODO:
    // /// Convenience wrapper around evaluate `Print`.
    // fn print(&self, args: impl Into<PrintArgs>);
}

pub fn initialize(libdata: WolframLibraryData) {
    let engine = WolframEngine::from(libdata);

    let link_name = engine.get_wstp_link().name();

    let set_print_full_symbols =
        unsafe { Symbol::unchecked_new("MathLink`LinkSetPrintFullSymbols") };

    let init = Expr! {
        Module[{link},
            // link = Select[Links[], Function[link, link[[1]] === 'link_name]][[1]];
            // 'set_print_full_symbols[link, True]
            Map[
                Function[link, 'set_print_full_symbols[link, True]],
                Links[]
            ]
        ]
    };

    engine.try_evaluate(&Expr! {
        Print[Length[Links[]]]
    });

    match engine.try_evaluate(&init) {
        Ok(res) => panic!(),
        Err(message) => {
            engine.try_evaluate(&Expr! {
                Print['message, " : ", 'link_name]
            });
        },
    }
}

// Because the wrapper is generated by macro, it's not necessary for LibraryLink to have a
// stable ABI?

// pub struct EvaluationData {
//     pub value: Expr,
//     /// Message that were generated during the evaluation.
//     pub message: Vec<forms::Message>,
// }

//======================================
// LibraryLinkStatus
//======================================

#[derive(Copy, Clone, Debug)]
pub enum LibraryLinkStatus {
    NoError,
    FunctionError,
    TypeError,
}

impl From<LibraryLinkStatus> for u32 {
    fn from(status: LibraryLinkStatus) -> u32 {
        use self::sys::{LIBRARY_FUNCTION_ERROR, LIBRARY_TYPE_ERROR};

        match status {
            LibraryLinkStatus::NoError => LIBRARY_NO_ERROR,
            LibraryLinkStatus::FunctionError => LIBRARY_FUNCTION_ERROR,
            LibraryLinkStatus::TypeError => LIBRARY_TYPE_ERROR,
        }
    }
}

// impl Try for LibraryLinkStatus {
//     type Ok = ();
//     type Error = Self;

//     fn into_result(self) -> Result<Self::Ok, Self::Error> {
//         match self {
//             LibraryLinkStatus::NoError => Ok(()),
//             s @ LibraryLinkStatus::FunctionError => Err(s),
//             s @ LibraryLinkStatus::TypeError => Err(s),
//         }
//     }

//     fn from_error(err: Self) -> Self {
//         match err {
//             LibraryLinkStatus::NoError => {
//                 panic!("Try::from_error for LibraryLinkStatus: got NoError")
//             },
//             LibraryLinkStatus::FunctionError | LibraryLinkStatus::TypeError => err,
//         }
//     }

//     fn from_ok(_ok: ()) -> Self {
//         LibraryLinkStatus::NoError
//     }
// }

// TODO: Allow any type which implements FromExpr in wrapper parameter lists?

//======================================
// #[wolfram_library_function] helpers
//======================================

/// Private.
///
/// Helper function used to implement the
/// [`wolfram_library_function`][macro@wolfram_library_function] macro.
pub fn call_wolfram_library_function(
    libdata: WolframLibraryData,
    unsafe_link: wstp::sys::WSLINK,
    function: fn(&WolframEngine, Vec<Expr>) -> Expr,
) -> std::os::raw::c_uint {
    use self::{
        catch_panic::{call_and_catch_panic, CaughtPanic},
        wstp::sys::{WSEndPacket, WSPutString},
    };

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(|| {
            // Contruct the engine
            let engine = WolframEngine::from(libdata);

            let link = WSTPLink::new(unsafe_link);

            let arguments: Expr = match link.get_expr() {
                Ok(args) => args,
                Err(message) => {
                    let _: Result<_, _> = link.put_expr(&Expr! {
                        Failure["LibraryFunctionWSTPError", <|
                            "Message" -> %[Expr::string(message)]
                        |>]
                    });
                    return;
                },
            };

            let arguments = match arguments.to_kind() {
                ExprKind::Normal(normal) => normal.contents,
                _ => panic!("WSTP argument expression was non-Normal"),
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
            use wl_lang::forms::ToPrettyExpr;
            // FIXME: Fix unwraps + return this as a full expr
            let cstring =
                CString::new(caught_panic.to_pretty_expr().to_string()).unwrap();

            WSPutString(unsafe_link, cstring.as_ptr());

            WSEndPacket(unsafe_link);

            LIBRARY_NO_ERROR
        },
    }
}
