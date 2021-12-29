use std::os::raw::c_uint;

use wl_expr_core::{Expr, ExprKind, Symbol};
use wstp::{self, Link};

use crate::{
    catch_panic::{call_and_catch_panic, CaughtPanic},
    sys::{self, MArgument, LIBRARY_NO_ERROR},
    NativeFunction, NumericArray,
};


//======================================
// #[wolfram_library_function] helpers
//======================================

//==================
// WSTP Expr helpers
//==================

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wstp_expr_list_wolfram_library_function(
    libdata: sys::WolframLibraryData,
    unsafe_link: wstp::sys::WSLINK,
    function: fn(Vec<Expr>) -> Expr,
) -> c_uint {
    call_wstp_expr_wolfram_library_function(
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
pub fn call_wstp_expr_wolfram_library_function<
    F: FnOnce(Expr) -> Expr + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    unsafe_link: wstp::sys::WSLINK,
    function: F,
) -> c_uint {
    call_wstp_link_wolfram_library_function(libdata, unsafe_link, |link: &mut Link| {
        let arguments: Expr = match link.get_expr() {
            Ok(args) => args,
            Err(message) => {
                // Skip reading the argument list packet.
                if link.raw_get_next().and_then(|_| link.new_packet()).is_err() {
                    return;
                }

                // Failure["LibraryFunctionWSTPError", <|
                //     "Message" -> %[Expr::string(message.to_string())]
                // |>]
                let failure = Expr::normal(Symbol::new("System`Failure").unwrap(), vec![
                    Expr::string("LibraryFunctionWSTPError"),
                    Expr::normal(Symbol::new("System`Association").unwrap(), vec![
                        Expr::normal(Symbol::new("System`Rule").unwrap(), vec![
                            Expr::string("Message"),
                            Expr::string(message.to_string()),
                        ]),
                    ]),
                ]);

                let _: Result<_, _> = link.put_expr(&failure);
                return;
            },
        };

        let result: Expr = function(arguments);

        link.put_expr(&result).expect(
            "LibraryFunction result expression could not be written to WSTP link",
        );
    })
}

//==================
// WSTP helpers
//==================

/// Private. Helper function used to implement [`#[wolfram_library_function]`][wlf] .
///
/// [wlf]: attr.wolfram_library_function.html
pub fn call_wstp_link_wolfram_library_function<
    F: FnOnce(&mut Link) + std::panic::UnwindSafe,
>(
    libdata: sys::WolframLibraryData,
    mut unsafe_link: wstp::sys::WSLINK,
    function: F,
) -> c_uint {
    let _ = unsafe { crate::initialize(libdata) };

    let link = unsafe { Link::unchecked_ref_cast_mut(&mut unsafe_link) };

    let result: Result<(), CaughtPanic> = unsafe {
        call_and_catch_panic(std::panic::AssertUnwindSafe(|| {
            let _: () = function(link);
        }))
    };

    match result {
        Ok(()) => LIBRARY_NO_ERROR,
        // Try to fail gracefully by writing the panic message as a Failure[..] object to
        // be returned, but if that fails, just return LIBRARY_FUNCTION_ERROR.
        Err(panic) => match write_panic_failure_to_link(link, panic) {
            Ok(()) => LIBRARY_NO_ERROR,
            Err(_wstp_err) => {
                // println!("PANIC ERROR: {}", _wstp_err);
                sys::LIBRARY_FUNCTION_ERROR // +1
            },
        },
    }
}

fn write_panic_failure_to_link(
    link: &mut Link,
    caught_panic: CaughtPanic,
) -> Result<(), wstp::Error> {
    // Clear the last error on the link, if any.
    //
    // This is necessary because the panic we caught might have been caused by
    // code like:
    //
    //     link.do_something(...).unwrap()
    //
    // where `do_something()` fails, which will have "poisoned" the link, and would cause
    // our attempt to write the panic message to the link to fail if we didn't clear the
    // error.
    //
    // If there is no error condition set on the link, this is a no-op.
    //
    // TODO: If an error *is* set, mention that in the Failure message? That might help
    //       users debug link issues more quickly.
    link.clear_error();

    // Skip whatever data is still stored in the link, if any.
    if link.is_ready() {
        link.raw_get_next()?;
        link.new_packet()?;
    }

    link.put_expr(&caught_panic.to_pretty_expr())
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
    let _ = unsafe { crate::initialize(libdata) };

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

//======================================
// Automatic Loader
//======================================

pub enum LibraryLinkFunction {
    Native {
        name: &'static str,
        /// # Implementation note on the type of this field
        ///
        /// In an ideal world, the type of this field would be something like
        /// `ty: Box<dyn NativeFunction>`.
        ///
        /// Using `fn() -> _` as the type of this field is necessary to work around
        /// the following constraints :
        ///
        /// * Instances of `LibraryLinkFunction` are constructed within a `static` context,
        ///   so only operations that are allowed in a `static` context can be used.
        ///
        /// * Can't be `&'static dyn for<'a> NativeFunction<'a>>`
        ///   - Doesn't work because it would require an intermediate `&'static fn(..)`
        ///     value, which can only be derived from an explicit `static FUNC: fn(..)`,
        ///     which in turn needs to be declared using explicit types for the function
        ///     parameter and return types (`static FUNC: fn(_, _) -> _` is not allowed,
        ///     because type inferrence doesn't work on static variables).
        ///
        /// * Can't be `Box<dyn for<'a> NativeFunction<'a>>`.
        ///   - Doesn't work because `Box::new()` can't be used in a `static` context.
        ///
        /// * Can't be `fn() -> Box<dyn NativeFunction<'a>>` because the `'a` lifetime
        ///   parameter can't be declared in any way.
        ///
        /// So in the end, we just call `NativeFunction::signature()` within `fn()`
        /// that is constructed in the macro-generated code (and where the concrete
        /// function type is still available) to avoid trying and failing to box up or
        /// return the `NativeFunction` trait object.
        signature: fn() -> Result<(Vec<Expr>, Expr), String>,
    },
    Wstp {
        name: &'static str,
    },
}

inventory::collect!(LibraryLinkFunction);

pub unsafe fn load_library_functions_impl(
    lib_data: sys::WolframLibraryData,
    raw_link: wstp::sys::WSLINK,
) -> c_uint {
    call_wstp_link_wolfram_library_function(lib_data, raw_link, |link: &mut Link| {
        let arg_count: usize =
            link.test_head("List").expect("expected 'List' expression");

        if arg_count != 1 {
            panic!(
                "expected 1 argument: the name of or file path to the dynamic library"
            );
        }

        let path = {
            let path = match link.get_string_ref() {
                Ok(value) => value,
                Err(err) => panic!("expected String argument (error: {})", err),
            };
            std::path::PathBuf::from(path.to_str())
        };

        let expr = library_function_load_expr(path);

        link.put_expr(&expr)
            .expect("failed to write loader Association");
    })
}

fn library_function_load_expr(library: std::path::PathBuf) -> Expr {
    let mut fields = Vec::new();
    let rule = Symbol::new("System`Rule").unwrap();

    for func in inventory::iter::<LibraryLinkFunction> {
        let code = match func.loading_code(&library) {
            Ok(code) => code,
            // TODO: Generate a message? Return a Failure[..]? Doing nothing seems
            //       reasonable too. This only currently fails for
            //       `fn(&[MArgument], MArgument)` functions.
            Err(_) => continue,
        };

        fields.push(Expr::normal(&rule, vec![Expr::string(func.name()), code]));
    }

    Expr::normal(Symbol::new("System`Association").unwrap(), fields)
}

impl LibraryLinkFunction {
    fn name(&self) -> &str {
        match self {
            LibraryLinkFunction::Native { name, .. } => name,
            LibraryLinkFunction::Wstp { name } => name,
        }
    }

    fn loading_code(&self, library: &std::path::PathBuf) -> Result<Expr, String> {
        let lib_func_load = Symbol::new("System`LibraryFunctionLoad").unwrap();
        let link_object = Expr::from(Symbol::new("System`LinkObject").unwrap());
        let library = Expr::string(
            library
                .to_str()
                .expect("unable to convert library file path to str"),
        );

        let code = match self {
            LibraryLinkFunction::Native { name, signature } => {
                let (args, ret) = signature()?;

                Expr::normal(&lib_func_load, vec![
                    library.clone(),
                    Expr::string(*name),
                    Expr::normal(Symbol::new("System`List").unwrap(), args),
                    ret,
                ])
            },
            // LibraryFunctionLoad[_,]
            LibraryLinkFunction::Wstp { name } => Expr::normal(&lib_func_load, vec![
                library.clone(),
                Expr::string(*name),
                link_object.clone(),
                link_object,
            ]),
        };

        Ok(code)
    }
}
