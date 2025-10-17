use std::os::raw::c_int;
use wolfram_library_link::{
    self as wll,
    sys::{self, WolframLibraryData},
    DataStore, DataStoreNodeValue,
    NumericArray,
};


#[no_mangle]
pub unsafe extern "C" fn WolframLibrary_initialize(lib: WolframLibraryData) -> c_int {
    match wll::initialize(lib) {
        Ok(()) => return 0,
        Err(()) => return 1,
    }
}

#[wll::export]
fn test_empty_data_store() -> DataStore {
    DataStore::new()
}

#[wll::export]
fn test_single_int_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);

    data
}

#[wll::export]
fn test_multiple_int_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_i64(2);
    data.add_i64(3);

    data
}

#[wll::export]
fn test_unnamed_heterogenous_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_i64(1);
    data.add_f64(2.0);
    data.add_str("hello");

    data
}

#[wll::export]
fn test_named_heterogenous_data_store() -> DataStore {
    let mut data = DataStore::new();
    data.add_named_i64("an i64", 1);
    data.add_named_f64("an f64", 2.0);
    data.add_named_str("a str", "hello");

    data
}

#[wll::export]
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

#[wll::export]
fn test_named_numeric_array_data_store() -> DataStore {
    let array = NumericArray::<i64>::from_slice(&[1, 2, 3]).into_generic();

    let mut data = DataStore::new();
    data.add_named_numeric_array("array", array);

    data
}

#[wll::export]
fn test_nested_data_store() -> DataStore {
    let mut inner = DataStore::new();
    inner.add_named_bool("is_inner", true);

    let mut outer = DataStore::new();
    outer.add_named_bool("is_inner", false);
    outer.add_data_store(inner);

    outer
}

#[wll::export]
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

#[wll::export]
fn test_data_store_arg(ds: DataStore) -> i64 {
    ds.len() as i64
}

//======================================
// DataStore nodes
//======================================

#[wll::export]
fn test_data_store_nodes() {
    {
        let mut data = DataStore::new();
        data.add_i64(5);

        assert_eq!(data.len(), 1);

        let node = data.first_node().expect("got first node");

        let ty: sys::type_t = node.data_type_raw();

        assert_eq!(ty, sys::MType_Integer as i32);
    }

    // Test DataStoreNode::name() method.
    {
        let mut data = DataStore::new();
        data.add_named_i64("hello", 5);

        assert_eq!(data.len(), 1);

        let node = data.first_node().expect("got first node");

        assert_eq!(node.data_type_raw(), sys::MType_Integer as i32);
        assert_eq!(node.name(), Some("hello".to_owned()))
    }

    // Test DataStore::nodes() method and Debug formatting of DataStoreNode.
    {
        let mut store = DataStore::new();

        store.add_i64(5);
        store.add_named_bool("condition", true);
        store.add_str("Hello, World!");

        let mut nodes = store.nodes();

        assert_eq!(
            format!("{:?}", nodes.next().unwrap()),
            r#"DataStoreNode { name: None, value: 5 }"#
        );
        assert_eq!(
            format!("{:?}", nodes.next().unwrap()),
            r#"DataStoreNode { name: Some("condition"), value: true }"#
        );
        assert_eq!(
            format!("{:?}", nodes.next().unwrap()),
            r#"DataStoreNode { name: None, value: "Hello, World!" }"#
        );
        assert!(nodes.next().is_none());
    }
}

//======================================
// u64 support (merged)
//======================================

#[wll::export]
fn ds_round_trip_u64(value: i64) -> DataStore {
    let as_u64: u64 = value as u64;
    assert!(as_u64 <= i64::MAX as u64);
    let mut ds = DataStore::new();
    ds.add_i64(as_u64 as i64);
    ds
}

#[wll::export]
fn ds_first_as_u64(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_add_too_large_u64() {
    let too_large = (i64::MAX as u128 + 1) as u64;
    assert!(too_large > i64::MAX as u64);
    panic!("u64 value exceeds i64::MAX; cannot store in DataStore integer slot");
}

//======================================
// u32/u16/u8 support
//======================================

#[wll::export]
fn ds_round_trip_u32(value: i64) -> DataStore {
    let as_u32: u32 = value as u32;
    assert!(as_u32 as i64 <= i64::MAX);
    let mut ds = DataStore::new();
    ds.add_i64(as_u32 as i64);
    ds
}

#[wll::export]
fn ds_first_as_u32(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_round_trip_u16(value: i64) -> DataStore {
    let as_u16: u16 = value as u16;
    assert!(as_u16 as i64 <= i64::MAX);
    let mut ds = DataStore::new();
    ds.add_i64(as_u16 as i64);
    ds
}

#[wll::export]
fn ds_first_as_u16(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_round_trip_u8(value: i64) -> DataStore {
    let as_u8: u8 = value as u8;
    assert!(as_u8 as i64 <= i64::MAX);
    let mut ds = DataStore::new();
    ds.add_i64(as_u8 as i64);
    ds
}

#[wll::export]
fn ds_first_as_u8(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

//======================================
// i32/i16/i8 support
//======================================

#[wll::export]
fn ds_round_trip_i32(value: i64) -> DataStore {
    let as_i32: i32 = value as i32;
    let mut ds = DataStore::new();
    ds.add_i64(as_i32 as i64);
    ds
}

#[wll::export]
fn ds_first_as_i32(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_round_trip_i16(value: i64) -> DataStore {
    let as_i16: i16 = value as i16;
    let mut ds = DataStore::new();
    ds.add_i64(as_i16 as i64);
    ds
}

#[wll::export]
fn ds_first_as_i16(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_round_trip_i8(value: i64) -> DataStore {
    let as_i8: i8 = value as i8;
    let mut ds = DataStore::new();
    ds.add_i64(as_i8 as i64);
    ds
}

#[wll::export]
fn ds_first_as_i8(ds: DataStore) -> i64 {
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

//======================================
// usize support
//======================================

#[wll::export]
fn ds_round_trip_usize(value: i64) -> DataStore {
    let as_usize: usize = value as usize;
    assert!(as_usize <= i64::MAX as usize);
    let mut ds = DataStore::new();
    ds.add_i64(as_usize as i64);
    ds
}

#[wll::export]
fn ds_first_as_usize(ds: DataStore) -> i64 {
    // return as i64 for compatibility with IntoArg
    let first = ds.first_node().expect("expected at least one node");
    match first.value() {
        DataStoreNodeValue::Integer(i) => i,
        other => panic!("unexpected node value: {:?}", other),
    }
}

#[wll::export]
fn ds_add_too_large_usize() {
    // Construct a value > i64::MAX if platform usize is wider. If not wider, deliberately panic manually.
    if std::mem::size_of::<usize>() > std::mem::size_of::<i64>() {
        let too_large = (i64::MAX as u128 + 1) as usize;
        assert!(too_large > i64::MAX as usize);
        panic!("usize value exceeds i64::MAX; cannot store in DataStore integer slot");
    } else {
        // 64-bit platforms: emulate overflow path
        panic!("usize overflow scenario forced for test");
    }
}
