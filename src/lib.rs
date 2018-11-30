#![feature(try_trait)]

use std::ffi::CStr;
use std::ops::Try;

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
// Macros
//======================================

#[macro_export]
macro_rules! link_wrapper {
    (fn $name:ident($lib_data:ident, $args:ident, $res:ident) -> LibraryLinkStatus $body:block) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe fn $name($lib_data: $crate::WolframLibraryData, arg_count: $crate::mint,
                            $args: *const $crate::MArgument, $res: $crate::MArgument)
                         -> u32 {
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
