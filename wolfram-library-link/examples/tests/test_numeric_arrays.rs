use wolfram_library_link::{self as wll, DataStore, NumericArray};

wll::export![
    // Scalar NumericArray's
    test_return_scalar_numeric_array();
    test_scalar_numeric_array_properties()
];

//--------------------------------------
// Scalar NumericArray's
//--------------------------------------

fn test_return_scalar_numeric_array() -> NumericArray {
    NumericArray::<i64>::from_scalar(5).into_generic()
}

fn test_scalar_numeric_array_properties() -> DataStore {
    let array: NumericArray<i64> = NumericArray::from_scalar(5);

    let mut store = DataStore::new();
    store.add_named_i64("rank", i64::try_from(array.rank()).unwrap());
    store.add_named_i64(
        "flattened_length",
        i64::try_from(array.flattened_length()).unwrap(),
    );
    store.add_named_i64(
        "dimensions.length()",
        i64::try_from(array.dimensions().len()).unwrap(),
    );

    store
}
