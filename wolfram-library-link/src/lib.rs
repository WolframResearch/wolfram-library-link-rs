//! A safe and ergonomic wrapper around Wolfram [LibraryLink][library-link-guide].
//!
//! Wolfram LibraryLink is an interface for dynamic libraries that can be
//! [dynamically loaded][LibraryFunctionLoad] by the [Wolfram Language][WL]. This crate
//! provides idiomatic Rust bindings around the lower-level LibraryLink C interface.
//!
//! **Features:**
//!
//! * Call Rust functions from Wolfram code.
//! * Pass data efficiently between Rust and Wolfram code using native data types
//!   like [`NumericArray`] and [`Image`].
//! * Pass arbitrary expressions between Rust and Wolfram code using
//!   [`Expr`][struct@crate::Expr] and the [`#[export(wstp)]`][crate::export#exportwstp]
//!   macro.
//! * Generate asynchronous events handled by the Wolfram Language, using an [`AsyncTaskObject`]
//!   background thread.
//!
//!
//!
//!
//! # What is Wolfram LibraryLink?
//!
//! > Wolfram LibraryLink provides a powerful way to connect external code to the Wolfram
//! > Language, enabling high-speed and memory-efficient execution. It does this by
//! > allowing dynamic libraries to be directly loaded into the Wolfram Language kernel so
//! > that functions in the libraries can be immediately called from the Wolfram Language.
//! >
//! > &nbsp;&nbsp;&nbsp;&nbsp;— [Wolfram LibraryLink User Guide](https://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html)
//!
//! ## Library function types
//!
//! The Wolfram LibraryLink interface supports two different types of library function:
//!
//! * Native functions
//! * WSTP functions
//!
//! The two function types differ in how their arguments and return value are passed
//! between the Wolfram Language and the compiled library function.
//!
//! Native functions pass their values using efficient native data types, like machine sized
//! integers, floating point numbers, C strings, or [`NumericArray`]s. Calling a native
//! function is efficient, but they are unable to pass more complicated values
//! (like general Wolfram Language expressions).
//!
//! WSTP functions pass their values using WSTP [`Link`] objects. A `Link` object can
//! be used to pass arbitrary Wolfram Language expressions to or from the library
//! function. However, WSTP links are less efficient for passing basic numeric arguments.
//!
//!
//!
//!
//! # Examples
//!
//! **Example programs:** In addition to the basic example programs show below, the
//! wolfram-library-link repository contains
//! [example programs](https://github.com/WolframResearch/wolfram-library-link-rs#example-programs)
//! demonstrating the major features of this crate.
//!
//! **Quick Start**: The [LibraryLink for Rust Quick Start][QuickStart] is a complete tutorial
//! covering how to create a new Rust library, compile it, and call into it from the
//! Wolfram Language.
//!
//! ## Native functions
//!
//! Rust functions can be exported for use from the Wolfram Language using the
//! [`#[export]`][crate::export] macro:
//!
//! ```
//! # mod scope {
//! use wolfram_library_link::export;
//!
//! #[export]
//! fn square(x: i64) -> i64 {
//!     x * x
//! }
//! # }
//! ```
//!
//! Then, after building the Rust code into a dynamic library, the function can be loaded
//! into the Wolfram Language using [`LibraryFunctionLoad`][LibraryFunctionLoad]:
//!
//! ```wolfram
//! square = LibraryFunctionLoad["<library name>", "square", {Integer}, Integer];
//! ```
//!
//! After being loaded, a library function can be called like any other Wolfram Language
//! function:
//!
//! ```wolfram
//! square[5]   (* Returns 25 *)
//! ```
//!
//! [QuickStart]: https://github.com/WolframResearch/wolfram-library-link-rs/blob/master/docs/QuickStart.md
//!
//! ## WSTP Functions
//!
//! Rust WSTP functions can be exported for use from the Wolfram Language using the
//! [`#[export(wstp)]`][crate::export#exportwstp] macro:
//!
//! ```
//! # mod scope {
//! use wolfram_library_link::{export, wstp::Link};
//!
//! #[export(wstp)]
//! fn square(link: &mut Link) {
//!     // WSTP function arguments are passed as a List expression: {...}
//!     let arg_count = link.test_head("System`List").unwrap();
//!
//!     // Get that the argument list contains a single element.
//!     if arg_count != 1 {
//!         panic!("square: expected one argument")
//!     }
//!
//!     // Check that the argument is an integer, and get it's value.
//!     let x: i64 = link.get_i64().expect("expected Integer argument");
//!
//!     // Write the return value.
//!     link.put_i64(x * x).unwrap();
//! }
//! # }
//! ```
//!
//!
//!
//!
//! # Additional Features
//!
//! ### Non-primitive native types
//!
//! Native functions support passing primitive types like [`bool`], [`i64`], [`f64`],
//! and strings. Additionally, they also support a small number of non-primitive types
//! that can be used to efficiently pass more complicated data structures between the
//! Wolfram Langauge and compiled code, without requiring the full generality of using a
//! WSTP function.
//!
//! The set of currently supported non-primitive native types includes:
//!
//! * [`NumericArray`]
//! * [`Image`]
//! * [`DataStore`]
//!
//! ### Cooperative computation abort handling
//!
//! The Wolfram Language supports the ability for the user to abort an in-progress
//! computation, without ending the process and losing their current state. This is
//! accomplished by code that checks if an abort has been triggered — and if so, performs an early return
//! — which is placed at important points in the Wolfram Language kernel
//! evaluation process.
//!
//! User libraries can cooperatively include abort checking logic in their library using
//! the [`aborted()`] function. This enables LibraryLink libraries to provide the same
//! user experience as built in Wolfram Language functions. LibraryLink libraries that may
//! perform long computations are especially encouraged to do abort checking within loops
//! that may run for a long time.
//!
//! ```no_run
//! use wolfram_library_link as wll;
//!
//! if wll::aborted() {
//!     // The user aborted this computation, so it doesn't matter what we return.
//!     panic!("Wolfram abort");
//! }
//! ```
//!
//! *Note: The Wolfram Language 'abort' command is not at all related to the
//! C/Rust [`abort()`][std::process::abort] function. The `abort()` function is typically used
//! when an unrecoverable error occurs, at a point determined by the programmer. The
//! Wolfram 'abort' command can be issued by the user at any point, and is commonly used
//! to end long-running computations the user no longer wishes to wait for.*
//!
//! ### Show backtrace when a panic occurs
//!
//! [WSTP functions](#wstp-functions) will automatically catch any
//! Rust panics that occur in the wrapped code, and return a [`Failure`][failure] object
//! with the panic message and source file/line number. The failure can optionally include
//! a backtrace showing the location of the panic.
//!
//! This is configured by the `"LIBRARY_LINK_RUST_BACKTRACE"` environment variable. Enable
//! it by evaluating:
//!
//! ```wolfram
//! SetEnvironment["LIBRARY_LINK_RUST_BACKTRACE" -> "True"]
//! ```
//!
//! Now the error shown when a panic occurs will include a backtrace.
//!
//! *The error message may include more information if the `"nightly"`
//! [feature][cargo-features] of `wolfram-library-link` is enabled.*
//!
//!
//!
//!
//! # Related Links
//!
//! * [LibraryLink for Rust Quick Start][QuickStart]
//! * [Wolfram LibraryLink User Guide](https://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html)
//!
//!
//!
//!
//! [WL]: https://wolfram.com/language
//! [library-link-guide]: https://reference.wolfram.com/language/guide/LibraryLink.html
//! [LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
//! [failure]: https://reference.wolfram.com/language/ref/Failure.html
//! [cargo-features]: https://doc.rust-lang.org/cargo/reference/features.html
// #![doc = include_str!("../docs/included/Overview.md")]
#![cfg_attr(feature = "nightly", feature(panic_info_message))]
#![warn(missing_docs)]

mod args;
mod async_tasks;
mod catch_panic;
mod data_store;
mod image;
mod library_data;
/// This module is *semver exempt*. This is not intended to be part of the public API of
/// wolfram-library-link.
///
/// Utilities used by code generated by the public macros.
#[doc(hidden)]
pub mod macro_utils;
pub mod managed;
mod numeric_array;
pub mod rtl;


// Note: This is exported as doc(inline) so that it shows up in the 'Modules' section of
//       the crate docs instead of in the 'Re-exports' section. This is to make way for
//       the chance that in the future, wolfram-library-link will have it's own expression
//       type that uses types like NumericArray and Image as variants, which can't be
//       used in the more general wolfram-expr crate (since NumericArray and Image depend
//       on the Wolfram RTL, which isn't available in arbitrary Rust code).
#[doc(inline)]
pub use wolfram_expr as expr;
pub use wolfram_library_link_sys as sys;
pub use wstp;

// Used by the #[export]/#[export(wstp)] macro implementations.
#[doc(hidden)]
pub use inventory;

pub use self::{
    args::{FromArg, IntoArg, NativeFunction, WstpFunction},
    async_tasks::AsyncTaskObject,
    data_store::{DataStore, DataStoreNode, DataStoreNodeValue, Nodes},
    image::{ColorSpace, Image, ImageData, ImageType, Pixel, UninitImage},
    library_data::{get_library_data, initialize, WolframLibraryData},
    numeric_array::{
        NumericArray, NumericArrayConvertMethod, NumericArrayDataType, NumericArrayKind,
        NumericArrayType, UninitNumericArray,
    },
};



use std::sync::Mutex;

use once_cell::sync::Lazy;

use wolfram_library_link_sys::mint;
use wstp::Link;

pub(crate) use self::library_data::assert_main_thread;
use crate::expr::{Expr, ExprKind, Symbol};

//--------------------------------------
// Re-exported items
//--------------------------------------

/// Designate an initialization function to run when this library is loaded via Wolfram
/// LibraryLink.
///
/// `#[init]` can be applied to at most one function in a library.
///
/// The function annotated with `#[init]` will automatically call [`initialize()`].
///
/// LibraryLink libraries are not required to define an initialization function.
///
/// # Panics
///
/// Any panics thrown during the executation of `#[init]` will automatically be caught,
/// and an error code will be returned to the Wolfram Kernel.
///
// TODO: Mention which error code specifically:
//         `macro_utils::error_code::FAILED_WITH_PANIC`.
///
/// If the initialization function panics, the Kernel will prevent other LibraryLink
/// functions exported from that library from being loaded.
///
/// # Example
///
/// Define an initialization function:
///
/// ```rust
/// use wolfram_library_link as wll;
///
/// #[wll::init]
/// fn init_my_library() {
///     println!("library is now initialized");
/// }
/// ```
///
/// # Behavior
///
/// If a library exports a function [called `WolframLibrary_initialize()`][lib-init], that
/// function will automatically be called by the Wolfram Kernel when the library is
/// loaded.
///
/// `#[init]` works by generating a definition for `WolframLibrary_initialize()`.
///
/// [lib-init]: https://reference.wolfram.com/language/LibraryLink/tutorial/LibraryStructure.html#280210622
pub use wolfram_library_link_macros::init;


/// Export the specified functions as native *LibraryLink* functions.
///
/// To be exported by this macro, the specified function(s) must implement
/// [`NativeFunction`].
///
/// Functions exported using this macro will automatically:
///
/// * Call [`initialize()`] to initialize this library.
/// * Catch any panics that occur.
///   - If a panic does occur, the function will return
///     [`LIBRARY_FUNCTION_ERROR`][crate::sys::LIBRARY_FUNCTION_ERROR].
///
// * Extract the function arguments from the raw [`MArgument`] array.
// * Store the function return value in the raw [`MArgument`] return value field.
///
/// # Syntax
///
/// Export a native function.
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::export;
/// #[export]
/// # fn square(x: i64) -> i64 { x }
/// # }
/// ```
///
/// Export a function using the specified low-level shared library symbol name.
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::export;
/// #[export(name = "WL_square")]
/// # fn square(x: i64) -> i64 { x }
/// # }
/// ```
///
/// # Examples
///
/// ### Primitive data types
///
/// Export a native function with a single integer argument:
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::export;
/// #[export]
/// fn square(x: i64) -> i64 {
///     x * x
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "square", {Integer}, Integer]
/// ```
///
/// Export a native function with a single string argument:
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::export;
/// #[export]
/// fn reverse_string(string: String) -> String {
///     string.chars().rev().collect()
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "reverse_string", {String}, String]
/// ```
///
/// Export a native function with multiple arguments:
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::export;
/// #[export]
/// fn times(a: f64, b: f64) -> f64 {
///     a * b
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "times", {Real, Real}, Real]
/// ```
///
/// ### Numeric arrays
///
/// Export a native function with a [`NumericArray`] argument:
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::{export, NumericArray};
/// #[export]
/// fn total_i64(list: &NumericArray<i64>) -> i64 {
///     list.as_slice().into_iter().sum()
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "...", "total_i64",
///     {LibraryDataType[NumericArray, "Integer64"]}
///     Integer
/// ]
/// ```
///
/// ### Customize exported function name
///
/// By default, the exported name of a function exported to the Wolfram Langauge
/// using `#[export]` will be the same as the Rust function name. The exported name
/// can be customed by specifying the `name = "..."` argument to the `#[export]` macro:
///
/// ```
/// # mod scope {
/// use wolfram_library_link::export;
///
/// #[export(name = "native_square")]
/// fn square(x: i64) -> i64 {
///     x * x
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["<library name>", "native_square", {Integer}, Integer]
/// ```
///
///
// TODO: Add a "Memory Management" section to this comment and discuss "Constant".
//
// ```wolfram
// LibraryFunctionLoad[
//     "...", "total_i64",
//     {
//         {LibraryDataType[NumericArray, "Integer64"], "Constant"}
//     },
//     Integer
// ]
// ```
///
/// # Parameter types
///
/// When manually writing the Wolfram
/// [`LibraryFunctionLoad`][ref/LibraryFunctionLoad]<sub>WL</sub> call necessary to load
/// a Rust *LibraryLink* function, you must declare the type signature of the function
/// using the appropriate types.
///
/// The following table describes the relationship between Rust types that can be used as
/// parameter types in a native LibraryLink function (namely: those that implement
/// [`FromArg`]) and the compatible Wolfram *LibraryLink* function parameter type(s).
///
/// [`FromArg::parameter_type()`] can be used to determine the Wolfram library function
/// parameter type programatically.
///
/// If you would prefer to have the Wolfram Language code for loading your library be
/// generated automatically, use the [`generate_loader!`] macro.
///
/// <h4 style="border-bottom: none; margin-bottom: 4px"> ⚠️ Warning! ⚠️ </h4>
///
/// Calling a *LibraryLink* function from the Wolfram Language that was loaded using the
/// wrong parameter type may lead to undefined behavior! Ensure that the function
/// parameter type declared in your Wolfram Language code matches the Rust function
/// parameter type.
///
/// Rust parameter type                | Wolfram library function parameter type
/// -----------------------------------|---------------------------------------
/// [`bool`]                           | `"Boolean"`
/// [`mint`]                           | `Integer`
/// [`mreal`][crate::sys::mreal]       | `Real`
/// [`mcomplex`][crate::sys::mcomplex] | `Complex`
/// [`String`]                         | `String`
/// [`CString`][std::ffi::CString]     | `String`
/// [`&NumericArray`][NumericArray]    | a. `LibraryDataType[NumericArray]` <br/> b. `{LibraryDataType[NumericArray], "Constant"}`[^1]
/// [`NumericArray`]                   | a. `{LibraryDataType[NumericArray], "Manual"}`[^1] <br/> b. `{LibraryDataType[NumericArray], "Shared"}`[^1]
/// [`&NumericArray<T>`][NumericArray] | a. `LibraryDataType[NumericArray, `[`"..."`][ref/NumericArray]`]`[^1] <br/> b. `{LibraryDataType[NumericArray, "..."], "Constant"}`[^1]
/// [`NumericArray<T>`]                | a. `{LibraryDataType[NumericArray, "..."], "Manual"}`[^1] <br/> b. `{LibraryDataType[NumericArray, "..."], "Shared"}`[^1]
/// [`DataStore`]                      | `"DataStore"`
///
/// # Return types
///
/// The following table describes the relationship between Rust types that implement
/// [`IntoArg`] and the compatible Wolfram *LibraryLink* function return type.
///
/// [`IntoArg::return_type()`] can be used to determine the Wolfram library function
/// parameter type programatically.
///
/// Rust return type                   | Wolfram library function return type
/// -----------------------------------|---------------------------------------
/// [`()`][`unit`]                     | `"Void"`
/// [`bool`]                           | `"Boolean"`
/// [`mint`]                           | `Integer`
/// [`mreal`][crate::sys::mreal]       | `Real`
/// [`i8`], [`i16`], [`i32`]           | `Integer`
/// [`u8`], [`u16`], [`u32`]           | `Integer`
/// [`f32`]                            | `Real`
/// [`mcomplex`][crate::sys::mcomplex] | `Complex`
/// [`String`]                         | `String`
/// [`NumericArray`]                   | `LibraryDataType[NumericArray]`
/// [`NumericArray<T>`]                | `LibraryDataType[NumericArray, `[`"..."`][ref/NumericArray][^1]`]`
/// [`DataStore`]                      | `"DataStore"`
///
/// [^1]: The Details and Options section of the Wolfram Language
///       [`NumericArray` reference page][ref/NumericArray] lists the available element
///       types.
///
/// [ref/NumericArray]: https://reference.wolfram.com/language/ref/NumericArray.html
/// [ref/LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
///
///
///
/// <br/><br/><br/>
///
/// # `#[export(wstp)]`
///
/// Export the specified functions as native *LibraryLink* WSTP functions.
///
/// To be exported by this macro, the specified function(s) must implement
/// [`WstpFunction`].
///
/// Functions exported using this macro will automatically:
///
/// * Call [`initialize()`][crate::initialize] to initialize this library.
/// * Catch any panics that occur.
///   - If a panic does occur, it will be returned as a [`Failure[...]`][ref/Failure]
///     expression.
///
/// [ref/Failure]: https://reference.wolfram.com/language/ref/Failure.html
///
/// # Syntax
///
/// Export a LibraryLink WSTP function.
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::{export, wstp::Link, expr::Expr};
/// #[export(wstp)]
/// # fn square(args: Vec<Expr>) -> Expr { todo!() }
/// # }
/// ```
///
/// Export a LibraryLink WSTP function using the specified low-level shared library symbol
/// name.
///
/// ```
/// # mod scope {
/// # use wolfram_library_link::{export, wstp::Link, expr::Expr};
/// #[export(wstp, name = "WL_square")]
/// # fn square(args: Vec<Expr>) -> Expr { todo!() }
/// # }
/// ```
///
/// # Examples
///
/// ##### WSTP function that squares a single integer argument:
///
/// ```
/// # mod scope {
/// use wolfram_library_link::{export, wstp::Link};
///
/// #[export(wstp)]
/// fn square_wstp(link: &mut Link) {
///     // Get the number of elements in the arguments list.
///     let arg_count = link.test_head("List").unwrap();
///
///     if arg_count != 1 {
///         panic!("square_wstp: expected to get a single argument");
///     }
///
///     // Get the argument value.
///     let x = link.get_i64().expect("expected Integer argument");
///
///     // Write the return value.
///     link.put_i64(x * x).unwrap();
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "square_wstp", LinkObject, LinkObject]
/// ```
///
/// ##### WSTP function that computes the sum of a variable number of arguments:
///
/// ```
/// # mod scope {
/// use wolfram_library_link::{export, wstp::Link};
///
/// #[export(wstp)]
/// fn total_args_i64(link: &mut Link) {
///     // Check that we recieved a functions arguments list, and get the number of arguments.
///     let arg_count: usize = link.test_head("List").unwrap();
///
///     let mut total: i64 = 0;
///
///     // Get each argument, assuming that they are all integers, and add it to the total.
///     for _ in 0..arg_count {
///         let term = link.get_i64().expect("expected Integer argument");
///         total += term;
///     }
///
///     // Write the return value to the link.
///     link.put_i64(total).unwrap();
/// }
/// # }
/// ```
///
/// ```wolfram
/// LibraryFunctionLoad["...", "total_args_i64", LinkObject, LinkObject]
/// ```
pub use wolfram_library_link_macros::export;

const BACKTRACE_ENV_VAR: &str = "LIBRARY_LINK_RUST_BACKTRACE";

//======================================
// Callbacks to the Wolfram Kernel
//======================================

/// Evaluate `expr` by calling back into the Wolfram Kernel.
///
/// TODO: Specify and document what happens if the evaluation of `expr` triggers a
///       kernel abort (such as a `Throw[]` in the code).
pub fn evaluate(expr: &Expr) -> Expr {
    match try_evaluate(expr) {
        Ok(returned) => returned,
        Err(msg) => panic!(
            "evaluate(): evaluation of expression failed: {}: \n\texpression: {}",
            msg, expr
        ),
    }
}

/// Attempt to evaluate `expr`, returning an error if a WSTP transport error occurred
/// or evaluation failed.
pub fn try_evaluate(expr: &Expr) -> Result<Expr, String> {
    with_link(|link: &mut Link| {
        // Send an EvaluatePacket['expr].
        let _: () = link
            // .put_expr(&Expr! { EvaluatePacket['expr] })
            .put_expr(&Expr::normal(Symbol::new("System`EvaluatePacket"), vec![
                expr.clone(),
            ]))
            .map_err(|e| e.to_string())?;

        let _: () = process_wstp_link(link)?;

        let return_packet: Expr = link.get_expr().map_err(|e| e.to_string())?;

        let returned_expr = match return_packet.kind() {
            ExprKind::Normal(normal) => {
                debug_assert!(normal.has_head(&Symbol::new("System`ReturnPacket")));
                debug_assert!(normal.elements().len() == 1);
                normal.elements()[0].clone()
            },
            _ => {
                return Err(format!(
                    "try_evaluate(): returned expression was not ReturnPacket: {}",
                    return_packet
                ))
            },
        };

        Ok(returned_expr)
    })
}

/// Returns `true` if the user has requested that the current evaluation be aborted.
///
/// Programs should finish what they are doing and return control of this thread to
/// to the kernel as quickly as possible. They should not exit the process or
/// otherwise terminate execution, simply return up the call stack.
///
/// Within Rust functions exported using [`#[export]`][crate::export] or
/// [`#[export(wstp)]`][crate::export#exportwstp] (which generate a wrapper function that
/// catches panics), `panic!()` can be used to quickly unwind the call stack to the
/// appropriate place.
/// Note that this will not work if the current library is built with
/// `panic = "abort"`. See the [`panic`][panic-option] profile configuration option
/// for more information.
///
/// [panic-option]: https://doc.rust-lang.org/cargo/reference/profiles.html#panic
pub fn aborted() -> bool {
    // TODO: Is this function thread safe? Can it be called from a thread other than the
    //       one the LibraryLink wrapper was originally invoked from?
    let val: mint = unsafe { rtl::AbortQ() };
    // TODO: What values can `val` be?
    val == 1
}

fn process_wstp_link(link: &mut Link) -> Result<(), String> {
    assert_main_thread();

    let raw_link = unsafe { link.raw_link() };

    // Process the packet on the link.
    let code: i32 = unsafe { rtl::processWSLINK(raw_link as *mut _) };

    if code == 0 {
        let error_message = link
            .error_message()
            .unwrap_or_else(|| "unknown error occurred on WSTP Link".into());

        return Err(error_message);
    }

    Ok(())
}

/// Enforce exclusive access to the link returned by `getWSLINK()`.
fn with_link<F: FnOnce(&mut Link) -> R, R>(f: F) -> R {
    assert_main_thread();

    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Default::default());

    let _guard = LOCK.lock().expect("failed to acquire LINK lock");

    let lib = get_library_data().raw_library_data;

    let unsafe_link: sys::WSLINK = unsafe { rtl::getWSLINK(lib) };
    let mut unsafe_link: wstp::sys::WSLINK = unsafe_link as wstp::sys::WSLINK;

    // Safety:
    //      By using LOCK to ensure exclusive access to the `getWSLINK()` value within
    //      safe code, we can be confident that this `&mut Link` will not alias with
    //      other references to the underling link object.
    let link = unsafe { Link::unchecked_ref_cast_mut(&mut unsafe_link) };

    f(link)
}

#[inline]
fn bool_from_mbool(boole: sys::mbool) -> bool {
    boole != 0
}


// TODO: Allow any type which implements FromExpr in wrapper parameter lists?

/// Generate and export a "loader" function, which returns an Association containing the
/// names and loaded forms of all functions exported by this library.
///
/// All functions exported by the [`#[export(..)]`][crate::export] macro will
/// automatically be included in the Association returned by this function.
///
/// # Syntax
///
/// Generate and export an automatic loader function.
///
/// ```
/// # use wolfram_library_link::generate_loader;
/// generate_loader![load_my_library];
/// ```
///
/// # Example
///
/// The following Rust program exports three primary functions via LibraryLink:
///
/// * `add2`
/// * `flat_total_i64`
/// * `time_since_epoch`
///
/// These functions are exported from the library using the
/// [`#[export(..)]`][crate::export] macro. This makes them loadable using
/// [`LibraryFunctionLoad`][ref/LibraryFunctionLoad]<sub>WL</sub>.
///
///
/// ```
/// # mod scope {
/// use wolfram_library_link::{self as wll, NumericArray, expr::{Expr, Symbol}};
///
/// wll::generate_loader![load_my_library_functions];
///
/// #[wll::export]
/// fn add2(x: i64, y: i64) -> i64 {
///     x + y
/// }
///
/// #[wll::export]
/// fn flat_total_i64(list: &NumericArray<i64>) -> i64 {
///     list.as_slice().into_iter().sum()
/// }
///
/// #[wll::export(wstp)]
/// fn time_since_epoch(args: Vec<Expr>) -> Expr {
///     use std::time::{SystemTime, UNIX_EPOCH};
///
///     assert!(args.len() == 0, "expected no arguments, got {}", args.len());
///
///     let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
///
///     Expr::normal(Symbol::new("System`Quantity"), vec![
///         Expr::real(duration.as_secs_f64()),
///         Expr::string("Seconds")
///     ])
/// }
/// # }
/// ```
///
/// However, writing out the correct invocations of
/// [`LibraryFunctionLoad`][ref/LibraryFunctionLoad] can be tedious and error prone.
/// `generate_loader!` provides an easier alternative way to load the functions
/// exported by this library.
///
/// In addition to the three previously mentioned functions, this library also exports a
/// fourth function, called `load_my_library_functions`.
///
/// Instead of writing three separate `LibraryFunctionLoad` calls, one for each exported
/// function, you can instead load the single `load_my_library_functions` function, which,
/// when called, will automatically load the other three functions exported by this
/// library:
///
/// ```wolfram
/// library = "example_library";
///
/// loadFunctions = LibraryFunctionLoad[library, "load_my_library_functions", LinkObject, LinkObject];
///
/// functions = loadFunctions[library];
/// ```
///
/// The `functions` variable will be an association, with roughly the following content:
///
/// ```wolfram
/// <|
///     "add2" -> LibraryFunction["example_library", "add2", {Integer, Integer}, Integer],
///     "flat_total_i64" -> LibraryFunction[
///         "example_library",
///         "flat_total_i64",
///         {{LibraryDataType[NumericArray, "Integer64"], "Constant"}},
///         Integer
///     ],
///     "time_since_epoch" -> LibraryFunction["example_library", "time_since_epoch", LinkObject]
/// |>
/// ```
///
/// As shown above, the `load_my_library_functions` function generated by
/// `generate_loader!` has automatically mapped the Rust paramater and return types onto
/// the appropriate Wolfram LibraryLink types.
///
/// Functions from the library can be called by applying arguments to the appropriate
/// value from the `functions` association:
///
/// ```wolfram
/// (* Returns 12 *)
/// functions["add2"][4, 8]
///
/// (* Returns 6 *)
/// functions["flat_total_i64"][NumericArray[{1, 2, 3}, "Integer64"]]
///
/// (* Returns Quantity[seconds_, "Seconds"], containing the current number of seconds
///    since the Unix epoch time. *)
/// functions["time_since_epoch"][]
/// ```
///
/// [ref/LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
#[macro_export]
macro_rules! generate_loader {
    ($name:ident) => {
        // TODO: Use this anonymous `const` trick in #[export(..)] too.
        const _: () = {
            #[no_mangle]
            pub unsafe extern "C" fn $name(
                lib: $crate::sys::WolframLibraryData,
                raw_link: $crate::wstp::sys::WSLINK,
            ) -> std::os::raw::c_uint {
                $crate::macro_utils::load_library_functions_impl(lib, raw_link)
            }
        };
    };
}
