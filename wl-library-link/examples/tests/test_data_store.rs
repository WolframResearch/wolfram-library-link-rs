use std::os::raw::{c_int, c_uint};
use wl_library_link::{
    self as wll,
    sys::{mint, MArgument, WolframLibraryData, LIBRARY_NO_ERROR},
    DataStore, NumericArray,
};

#[no_mangle]
pub unsafe extern "C" fn WolframLibrary_initialize(lib: WolframLibraryData) -> c_int {
    match wll::initialize(lib) {
        Ok(()) => return 0,
        Err(()) => return 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_empty_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    *res.tensor = DataStore::new().into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_single_int_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut data = DataStore::new();
    data.add_i64(1);

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_multiple_int_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_i64(2);
    data.add_i64(3);

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_unnamed_heterogenous_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_f64(2.0);
    data.add_str("hello");

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_named_heterogenous_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut data = DataStore::new();
    data.add_named_i64("an i64", 1);
    data.add_named_f64("an f64", 2.0);
    data.add_named_str("a str", "hello");

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_named_and_unnamed_heterogenous_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_named_f64("real", 2.0);
    data.add_named_str("hello", "world");

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

//======================================
// Non-atomic types
//======================================

#[no_mangle]
pub unsafe extern "C" fn test_named_numeric_array_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let array = NumericArray::<i64>::from_slice(&[1, 2, 3]).into_generic();

    let mut data = DataStore::new();
    data.add_named_numeric_array("array", array);

    *res.tensor = data.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_nested_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut inner = DataStore::new();
    inner.add_named_bool("is_inner", true);

    let mut outer = DataStore::new();
    outer.add_named_bool("is_inner", false);
    outer.add_data_store(inner);

    *res.tensor = outer.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}

#[no_mangle]
pub unsafe extern "C" fn test_iterated_nested_data_store(
    _: WolframLibraryData,
    _: mint,
    _: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let mut store = DataStore::new();

    for level in 0..3 {
        store.add_named_i64("level", level);
        let mut new = DataStore::new();
        new.add_data_store(store);
        store = new;
    }

    *res.tensor = store.into_raw() as *mut _;

    LIBRARY_NO_ERROR
}
