use std::os::raw::{c_int, c_uint};

use wl_library_link::{
    self as wll,
    sys::{
        self, mint, MArgument, WolframLibraryData, LIBRARY_NO_ERROR,
        LIBRARY_NUMERICAL_ERROR, LIBRARY_TYPE_ERROR,
    },
    NumericArray, NumericArrayKind,
};

#[no_mangle]
extern "C" fn WolframLibrary_initialize(data: sys::WolframLibraryData) -> c_int {
    match wll::initialize(data) {
        Ok(()) => return 0,
        Err(()) => return 1,
    }
}

/// This function is loaded by evaluating:
///
/// ```wolfram
/// LibraryFunctionLoad[
///     "/path/to/libnumeric_arrays.dylib",
///     "sum_int_numeric_array",
///     {NumericArray},
///     Integer
/// ]
/// ```
#[no_mangle]
pub extern "C" fn sum_int_numeric_array(
    _: WolframLibraryData,
    arg_count: mint,
    args: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let args: &mut [MArgument] =
        unsafe { std::slice::from_raw_parts_mut(args, arg_count as usize) };

    let na = unsafe { NumericArray::from_raw(*args[0].numeric) };

    #[rustfmt::skip]
    let sum: i64 = match na.kind() {
        NumericArrayKind::Bit8(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit16(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit32(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit64(na) => na.data().into_iter().sum(),
        NumericArrayKind::UBit8(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit16(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit32(na) => na.data().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit64(na) => {
            match i64::try_from(na.data().into_iter().sum::<u64>()) {
                Ok(sum) => sum,
                Err(_) => return LIBRARY_NUMERICAL_ERROR,
            }
        },

        NumericArrayKind::Real32(_)
        | NumericArrayKind::Real64(_)
        | NumericArrayKind::ComplexReal64(_) => return LIBRARY_TYPE_ERROR,
    };

    unsafe { *res.integer = sum };

    return LIBRARY_NO_ERROR;
}
