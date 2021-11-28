use std::ffi::{CStr, CString};

use crate::{
    sys::{self, mint, MArgument},
    NumericArray,
};

/// Unsafe trait used to implement conversion from an [`MArgument`].
/// See [`from_args()`].
pub trait FromArg {
    #[allow(missing_docs)]
    unsafe fn from_arg(arg: MArgument) -> Self;
}

/// Unsafe trait used to implement conversion from an [`MArgument`].
/// See [`from_args()`].
pub trait FromArgs {
    #[allow(missing_docs)]
    unsafe fn from_args(args: &[MArgument]) -> Self;
}

//======================================
// from_args()
//======================================

/// Utility for unwrapping [`MArgument`] arguments of a LibraryLink function.
///
/// If `T` is a tuple type, the elements of the returned tuple will be taken from
/// successive elements of `args`.
///
/// # Safety
///
/// [`from_args()`] will convert [`MArgument`] values into specified `T` type without
/// performing any validation. If the `args` passed to this function have not been
/// initialized to valid values for `T`, undefined behavior may occur.
///
/// # Examples
///
/// ```no_run
/// use std::os::raw::c_uint;
/// use wl_library_link::{from_args, NumericArray, sys::{WolframLibraryData, MArgument, mint}};
///
/// #[no_mangle]
/// pub unsafe extern "C" fn func(
///     _: WolframLibraryData,
///     argc: mint,
///     args: *mut MArgument,
///     res: MArgument
/// ) -> c_uint {
///     let (s, int, reals): (String, i64, NumericArray<f64>) = unsafe { from_args(args, argc) };
///
///     // ...
/// #   todo!()
/// }
/// ```
///
/// defines a LibraryLink function which could be loaded using:
///
/// ```wolfram
/// LibraryFunctionLoad[_, _, {String, Real, LibraryDataType[NumericArray, "Real64"]}, _]
/// ```
///
/// ## Single argument
///
/// ```no_run
/// use std::{os::raw::c_uint, ffi::CString};
/// use wl_library_link::{from_args, sys::{self, WolframLibraryData, MArgument, mint}};
///
/// #[no_mangle]
/// pub extern "C" fn string_bytes_length(
///     _: WolframLibraryData,
///     argc: mint,
///     args: *mut MArgument,
///     res: MArgument
/// ) -> c_uint {
///     let name: CString = unsafe { from_args(args, argc) };
///
///     unsafe {
///         *res.integer = name.to_bytes().len() as i64;
///     }
///
///     sys::LIBRARY_NO_ERROR
/// }
/// ```
///
/// defines a LibraryLink function which could be loaded using:
///
/// ```wolfram
/// LibraryFunctionLoad[_, _, {String}, Integer]
/// ```
///
/// ## Two arguments
///
/// ```no_run
/// use std::os::raw::c_uint;
/// use wl_library_link::{from_args, sys::{self, WolframLibraryData, MArgument, mint}};
///
/// #[no_mangle]
/// pub extern "C" fn sum(
///     _: WolframLibraryData,
///     argc: mint,
///     args: *mut MArgument,
///     res: MArgument
/// ) -> c_uint {
///     let (start, end): (i64, i64) = unsafe { from_args(args, argc) };
///
///     unsafe {
///         *res.integer = (start..end).sum();
///     }
///
///     sys::LIBRARY_NO_ERROR
/// }
/// ```
///
/// defines a LibraryLink function which could be loaded using:
///
/// ```wolfram
/// LibraryFunctionLoad[_, _, {Integer, Integer}, Integer]
/// ```
pub unsafe fn from_args<T: FromArgs>(args: *mut sys::MArgument, arg_count: mint) -> T {
    let arg_count = match usize::try_from(arg_count) {
        Ok(count) => count,
        Err(_) => panic!("from_args: argument count overflows usize"),
    };

    let args: &[MArgument] = std::slice::from_raw_parts(args, arg_count);

    T::from_args(args)
}

//======================================
// FromArg Impls
//======================================

impl FromArg for bool {
    unsafe fn from_arg(arg: MArgument) -> Self {
        *arg.boolean != 0
    }
}

impl FromArg for i64 {
    unsafe fn from_arg(arg: MArgument) -> Self {
        *arg.integer
    }
}

impl FromArg for f64 {
    unsafe fn from_arg(arg: MArgument) -> Self {
        *arg.real
    }
}

impl FromArg for sys::mcomplex {
    unsafe fn from_arg(arg: MArgument) -> Self {
        *arg.cmplex
    }
}

impl<T> FromArg for NumericArray<T> {
    unsafe fn from_arg(arg: MArgument) -> Self {
        NumericArray::from_raw(*arg.numeric)
    }
}

// NOT SAFE: The resulting lifetime is completely unconstrained.
// impl<'a> FromArg for &'a CStr {
//     unsafe fn from_arg(arg: MArgument) -> &'a CStr {
//         let cstr: *mut i8 = *arg.utf8string;
//         CStr::from_ptr(cstr)
//     }
// }

impl FromArg for CString {
    unsafe fn from_arg(arg: MArgument) -> CString {
        let cstr: *mut i8 = *arg.utf8string;
        let cstr = CStr::from_ptr(cstr);
        CString::from(cstr)
    }
}

impl FromArg for String {
    unsafe fn from_arg(arg: MArgument) -> String {
        let cstr: *mut i8 = *arg.utf8string;
        let cstr = CStr::from_ptr(cstr);
        cstr.to_str()
            .unwrap_or_else(|_| panic!("MArgument string is not valid UTF-8"))
            .to_owned()
    }
}

//======================================
// FromArgs Impls
//======================================

impl<A: FromArg> FromArgs for A {
    unsafe fn from_args(args: &[MArgument]) -> Self {
        if args.len() != 1 {
            panic!("from_args: expected 1 argument(s), got {}", args.len());
        }

        A::from_arg(*args.get_unchecked(0))
    }
}

macro_rules! tuple_impl {
    ($length:literal; $($type:ident),*) => {
        impl<$($type: FromArg),*> FromArgs for ($($type,)*) {
            unsafe fn from_args(args: &[MArgument]) -> Self {
                if args.len() != $length {
                    panic!("from_args: expected {} argument(s), got {}", $length, args.len());
                }

                let mut index: usize = 0;

                #[allow(unused_assignments)]
                (
                    $({
                        let val = $type::from_arg(*args.get_unchecked(index));
                        index += 1;
                        val
                    },)*
                )
            }
        }
    };
}

tuple_impl!( 1; A);
tuple_impl!( 2; A, B);
tuple_impl!( 3; A, B, C);
tuple_impl!( 4; A, B, C, D);
tuple_impl!( 5; A, B, C, D, E);
tuple_impl!( 6; A, B, C, D, E, F);
tuple_impl!( 7; A, B, C, D, E, F, G);
tuple_impl!( 8; A, B, C, D, E, F, G, H);
tuple_impl!( 9; A, B, C, D, E, F, G, H, I);
tuple_impl!(10; A, B, C, D, E, F, G, H, I, J);
