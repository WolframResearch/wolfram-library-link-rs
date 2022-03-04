use wolfram_library_link::{self as wll, NumericArray, NumericArrayKind};

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
#[wll::export]
fn sum_int_numeric_array(na: &NumericArray) -> i64 {
    #[rustfmt::skip]
    let sum: i64 = match na.kind() {
        NumericArrayKind::Bit8(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit16(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit32(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::Bit64(na) => na.as_slice().into_iter().sum(),
        NumericArrayKind::UBit8(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit16(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit32(na) => na.as_slice().into_iter().copied().map(i64::from).sum(),
        NumericArrayKind::UBit64(na) => {
            match i64::try_from(na.as_slice().into_iter().sum::<u64>()) {
                Ok(sum) => sum,
                Err(_) => panic!("NumericArray UnsignedInteger64 sum overflows i64"),
            }
        },

        NumericArrayKind::Real32(_)
        | NumericArrayKind::Real64(_)
        | NumericArrayKind::ComplexReal64(_) => panic!(
            "sum_int_numeric_array cannot handle non-integer data type: {:?}",
            na.data_type()
        ),
    };

    sum
}

#[wll::export]
fn sum_real_numeric_array(na: &NumericArray) -> f64 {
    let sum: f64 = match na.kind() {
        NumericArrayKind::Real32(na) => {
            na.as_slice().into_iter().copied().map(f64::from).sum()
        },
        NumericArrayKind::Real64(na) => na.as_slice().into_iter().copied().sum(),
        _ => panic!(
            "sum_real_numeric_array cannot handle non-real data type: {:?}",
            na.data_type()
        ),
    };

    sum
}
