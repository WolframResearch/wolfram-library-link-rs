use wolfram_library_link::{self as wll, DataStore, NumericArray};


#[wll::export]
fn test_na_automatic_count(array: &NumericArray) -> i64 {
    array.share_count() as i64
}

#[wll::export]
fn test_na_constant_count(array: &NumericArray) -> i64 {
    array.share_count() as i64
}

#[wll::export]
fn test_na_manual_count(array: NumericArray) -> i64 {
    array.share_count() as i64
}

#[wll::export]
fn test_na_shared_count(array: NumericArray) -> i64 {
    array.share_count() as i64
}

//

#[wll::export]
fn test_na_constant_are_ptr_eq(
    array1: &NumericArray<i64>,
    array2: &NumericArray<i64>,
) -> DataStore {
    let mut data = DataStore::new();
    data.add_bool(array1.ptr_eq(&array2));
    data.add_i64(array1.share_count() as i64);
    data
}

#[wll::export]
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

#[wll::export]
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

//----------------------------
// Test cloning NumericArray's
//----------------------------

#[wll::export]
fn test_na_clone() -> bool {
    let array = NumericArray::<i64>::from_slice(&[1, 2, 3]);

    assert!(array.share_count() == 0);

    let clone = array.clone();

    assert!(!array.ptr_eq(&clone));

    assert!(array.share_count() == 0);
    assert!(clone.share_count() == 0);

    true
}

#[wll::export]
fn test_na_shared_clone(array: NumericArray<i64>) -> bool {
    assert!(array.share_count() == 1);

    let clone = array.clone();

    assert!(!array.ptr_eq(&clone));

    assert!(array.share_count() == 1);
    assert!(clone.share_count() == 0);

    true
}
