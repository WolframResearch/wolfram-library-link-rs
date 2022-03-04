use wolfram_library_link::{
    self as wll, NumericArray, NumericArrayConvertMethod as Method,
};

fn from_slice<T: wll::NumericArrayType>(slice: &[T]) -> NumericArray<T> {
    NumericArray::from_slice(slice)
}

#[wll::export]
fn test_na_conversions() {
    //
    // i16 -> i8 conversions
    //

    assert!(from_slice(&[i16::MAX])
        .convert_to::<i8>(Method::Check, 1.0)
        .is_err());

    assert!(from_slice(&[i16::MAX])
        .convert_to::<i8>(Method::Cast, 1.0)
        .is_err());

    assert!(from_slice(&[i16::MAX])
        .convert_to::<i8>(Method::Coerce, 1.0)
        .is_err());

    assert!(from_slice(&[i16::MAX])
        .convert_to::<i8>(Method::Round, 1.0)
        .is_err());

    assert_eq!(
        from_slice(&[i16::MAX])
            .convert_to::<i8>(Method::Scale, 1.0)
            .unwrap()
            .as_slice(),
        [i8::MAX]
    );
}
