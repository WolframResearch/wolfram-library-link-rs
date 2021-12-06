use wl_library_link::{self as wll, DataStore, NumericArray};

wll::export![
    test_na_automatic_count(_);
    test_na_constant_count(_);
    test_na_manual_count(_);
    test_na_shared_count(_);
    //
    test_na_constant_are_ptr_eq(_, _);
    test_na_manual_are_not_ptr_eq(_, _);
    test_na_shared_are_ptr_eq(_, _);
];

fn test_na_automatic_count(array: &NumericArray) -> i64 {
    array.share_count() as i64
}

fn test_na_constant_count(array: &NumericArray) -> i64 {
    array.share_count() as i64
}

fn test_na_manual_count(array: NumericArray) -> i64 {
    array.share_count() as i64
}

fn test_na_shared_count(array: NumericArray) -> i64 {
    array.share_count() as i64
}

//

fn test_na_constant_are_ptr_eq(
    array1: &NumericArray<i64>,
    array2: &NumericArray<i64>,
) -> DataStore {
    let mut data = DataStore::new();
    data.add_bool(array1.ptr_eq(&array2));
    data.add_i64(array1.share_count() as i64);
    data
}

fn test_na_manual_are_not_ptr_eq(
    mut array1: NumericArray<i64>,
    array2: NumericArray<i64>,
) -> DataStore {
    let mut data = DataStore::new();
    data.add_bool(array1.ptr_eq(&array2));
    data.add_i64(array1.share_count() as i64);
    data.add_bool(array1.as_slice_mut().is_some());
    data
}

fn test_na_shared_are_ptr_eq(
    mut array1: NumericArray<i64>,
    array2: NumericArray<i64>,
) -> DataStore {
    let mut data = DataStore::new();
    data.add_bool(array1.ptr_eq(&array2));
    data.add_i64(array1.share_count() as i64);
    data.add_bool(array1.as_slice_mut().is_some());
    data
}
