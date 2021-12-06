use std::os::raw::c_int;
use wolfram_library_link::{
    self as wll, sys::WolframLibraryData, DataStore, NumericArray,
};


#[no_mangle]
pub unsafe extern "C" fn WolframLibrary_initialize(lib: WolframLibraryData) -> c_int {
    match wll::initialize(lib) {
        Ok(()) => return 0,
        Err(()) => return 1,
    }
}

wll::export![
    test_empty_data_store();
    test_single_int_data_store();
    test_multiple_int_data_store();
    test_unnamed_heterogenous_data_store();
    test_named_heterogenous_data_store();
    test_named_and_unnamed_heterogenous_data_store();
    test_named_numeric_array_data_store();
    test_nested_data_store();
    test_iterated_nested_data_store();
    test_data_store_arg(_);
];

fn test_empty_data_store() -> DataStore {
    DataStore::new()
}

fn test_single_int_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);

    data
}

fn test_multiple_int_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_i64(2);
    data.add_i64(3);

    data
}

fn test_unnamed_heterogenous_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_f64(2.0);
    data.add_str("hello");

    data
}

fn test_named_heterogenous_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_named_i64("an i64", 1);
    data.add_named_f64("an f64", 2.0);
    data.add_named_str("a str", "hello");

    data
}

fn test_named_and_unnamed_heterogenous_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_named_f64("real", 2.0);
    data.add_named_str("hello", "world");

    data
}

//======================================
// Non-atomic types
//======================================

fn test_named_numeric_array_data_store() -> DataStore {
    let array = NumericArray::<i64>::from_slice(&[1, 2, 3]).into_generic();

    let mut data = DataStore::new();
    data.add_named_numeric_array("array", array);

    data
}

fn test_nested_data_store() -> DataStore {
    let mut inner = DataStore::new();
    inner.add_named_bool("is_inner", true);

    let mut outer = DataStore::new();
    outer.add_named_bool("is_inner", false);
    outer.add_data_store(inner);

    outer
}

fn test_iterated_nested_data_store() -> DataStore {
    let mut store = DataStore::new();

    for level in 0..3 {
        store.add_named_i64("level", level);
        let mut new = DataStore::new();
        new.add_data_store(store);
        store = new;
    }

    store
}

//======================================
// DataStore arguments
//======================================

fn test_data_store_arg(ds: DataStore) -> i64 {
    ds.len() as i64
}
