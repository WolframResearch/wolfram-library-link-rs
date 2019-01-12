#![feature(try_trait)]

use std::ffi::{CStr, CString};
use std::ops::Try;

use wl_expr::{Expr, SymbolTable};
use wl_lang::sym;
use wl_parse;

pub use wl_library_link_sys::{
    WolframLibraryData, mint, MArgument,
    // Errors
    LIBRARY_NO_ERROR, LIBRARY_FUNCTION_ERROR, LIBRARY_TYPE_ERROR,
};

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
        match status {
            LibraryLinkStatus::NoError => LIBRARY_NO_ERROR,
            LibraryLinkStatus::FunctionError => LIBRARY_FUNCTION_ERROR,
            LibraryLinkStatus::TypeError => LIBRARY_TYPE_ERROR,
        }
    }
}

impl Try for LibraryLinkStatus {
    type Ok = ();
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            LibraryLinkStatus::NoError => Ok(()),
            s @ LibraryLinkStatus::FunctionError => Err(s),
            s @LibraryLinkStatus::TypeError => Err(s),
        }
    }

    fn from_error(err: Self) -> Self {
        match err {
            LibraryLinkStatus::NoError =>
                panic!("Try::from_error for LibraryLinkStatus: got NoError"),
            LibraryLinkStatus::FunctionError | LibraryLinkStatus::TypeError => err,
        }
    }

    fn from_ok(_ok: ()) -> Self {
        LibraryLinkStatus::NoError
    }
}

//======================================
// Utilities
//======================================

/// Set `res` to a "UTF8String" which is the printed form of a
/// `Failure[$kind, <| "Message" -> $err |>]`.
pub unsafe fn failure_msg(res: MArgument, kind: &str, msg: String) -> LibraryLinkStatus {
    let failure = failure_expr(kind, msg);
    write_expr(failure, res);
    LibraryLinkStatus::NoError
}

// TODO: Rename `err` to `message`.
pub fn failure_expr(kind: &str, err: String) -> Expr {
    let assoc = {
        let msg_rule = Expr::normal(*sym::Rule, vec![
            Expr::string("Message"), Expr::string(err)]);
        Expr::normal(*sym::Association, vec![msg_rule])
    };
    Expr::normal(*sym::Failure, vec![Expr::string(kind), assoc])
}

pub unsafe fn write_expr(expr: Expr, arg: MArgument) {
    // `Display for Expr` handles escaping any special characters which need it. This
    // string is therefore safe for consumption by ToExpression.
    let string = format!("{}", expr);
    // FIXME: This string is never freed
    let string = CString::new(string).unwrap();
    // FIXME: What happens if LibraryLink tries to free this string (which is
    //        currently leaked)?
    *arg.utf8string = string.into_raw();
}

/// Convert a "UTF8String" argument to `&str`.
pub unsafe fn marg_str(arg: &MArgument) -> Result<&str, LibraryLinkStatus> {
    let string: *const i8 = *arg.utf8string;
    let string = CStr::from_ptr(string);
    let string: &str = match string.to_str() {
        Ok(s) => s,
        Err(_) => return Err(LibraryLinkStatus::TypeError),
    };
    Ok(string)
}

/// Parse an `Expr` from an `MArgument` of type `"UTF8String"`.
///
/// Will return a `Failure["ParseError", _]` if parsing is unsuccesful. This should be
/// extremely rare, however, assuming the function is properly used.
pub fn marg_str_expr(string: &str) -> Result<Expr, Expr> {
    let mut st = SymbolTable::new("Global`", &[] as &[String]);
    match wl_parse::parse(&mut st, string) {
        Ok(expr) => Ok(expr),
        // TODO: Possible to show a message through LibraryLink?
        Err(err) => Err(failure_expr("ParseError", format!("{:?}", err))),
    }
}

//======================================
// Macros
//======================================

// TODO: Expose the LibraryLink printing function to users of this library. This will help
//       with debugging the compiler from the FE significantly.

#[macro_export]
macro_rules! link_wrapper {
    (fn $name:ident($lib_data:ident, $args:ident, $res:ident) -> LibraryLinkStatus $body:block) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe fn $name($lib_data: $crate::WolframLibraryData, arg_count: $crate::mint,
                            $args: *const $crate::MArgument, $res: $crate::MArgument)
                         -> u32 {
            use std::convert::TryFrom;

            let arg_count = match usize::try_from(arg_count) {
                Ok(count) => count,
                // NOTE: This will never happen as long as LibraryLink doesn't give us a
                //       negative argument count. If that happens, something else has
                //       gone seriously wrong, so let's do the least unhelpful thing.
                // TODO: Is there a better error we could return here?
                Err(_) => return $crate::LIBRARY_FUNCTION_ERROR,
            };
            let $args: &[$crate::MArgument] = ::std::slice::from_raw_parts($args, arg_count);
            let closure = || $body;
            let status: LibraryLinkStatus = closure();
            u32::from(status)
        }
    }
}

#[macro_export]
macro_rules! generate_wrapper {
    ($wrapper:ident # $func:ident ( $($arg:ident : Expr),* ) -> Expr) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe fn $wrapper(lib_data: $crate::WolframLibraryData,
                               arg_count: $crate::mint,
                               args: *const $crate::MArgument,
                               res: $crate::MArgument) -> u32 {
            use std::convert::TryFrom;
            use $crate::{
                marg_str, marg_str_expr, write_expr, LibraryLinkStatus,
                // Re-exported from wl-library-link-sys
                MArgument,
                LIBRARY_NO_ERROR, LIBRARY_FUNCTION_ERROR,
            };

            let arg_count = match usize::try_from(arg_count) {
                Ok(count) => count,
                // NOTE: This will never happen as long as LibraryLink doesn't give us a
                //       negative argument count. If that happens, something else has
                //       gone seriously wrong, so let's do the least unhelpful thing.
                // TODO: Is there a better error we could return here?
                Err(_) => return LIBRARY_FUNCTION_ERROR,
            };
            let margs: &[MArgument] = ::std::slice::from_raw_parts(args, arg_count);

            // Keep track of how many times $arg repeats, so we known which index in
            // `margs` to access.
            let mut arg_idx = 0;
            $(
                let marg = match margs.get(arg_idx) {
                    Some(marg) => marg,
                    /// This implies that the LibraryFunction wrapper in top-level does
                    /// not have enough arguments.
                    None => return LIBRARY_FUNCTION_ERROR,
                };
                let string = match marg_str(marg) {
                    Ok(s) => s,
                    Err(status) => return u32::from(status),
                };
                let $arg: Expr = match marg_str_expr(string) {
                    Ok(expr) => expr,
                    Err(expr) => {
                        write_expr(expr, res);
                        return LibraryLinkStatus::NoError.into()
                    },
                };

                arg_idx += 1;
            );*

            // PRE-COMMIT: wrap in catch_panic_message()
            let func: fn($($arg: Expr),*) -> Expr = $func;

            let res_expr: Expr = func($($arg),*);

            write_expr(res_expr, res);
            LIBRARY_NO_ERROR
        }
    }
}