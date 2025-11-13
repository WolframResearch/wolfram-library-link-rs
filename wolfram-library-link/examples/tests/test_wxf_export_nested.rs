//! Export a nested Vec<Vec<Option<(u64, BigUint)>>> structure as WXF bytes via
//! the polymorphic serializer. Consumed by a Wolfram `.wlt` test that does
//! BinaryDeserialize to recover structure.

use wolfram_library_link::{export, NumericArray};
use wolfram_library_link::wxf_poly::to_wxf_bytes;
use num_bigint::BigUint;

#[export(name = "test_wxf_export_nested_biguint")]
pub fn wxf_export_nested_biguint() -> NumericArray<u8> {
    // Construct nested data: {{ {42, big}, Null }, { Null, {7, 99} }} where
    // tuple -> List, None -> Null; BigUint large value exercises arbitrary precision.
    let big = BigUint::parse_bytes(b"123456789012345678901234567890", 10).unwrap();
    let data: Vec<Vec<Option<(u64, BigUint)>>> = vec![
        vec![Some((42u64, big.clone())), None],
        vec![None, Some((7u64, BigUint::from(99u32)))]
    ];
    let bytes = to_wxf_bytes(&data).unwrap_or_default();
    NumericArray::from_slice(bytes.as_slice())
}

#[export(name = "test_wxf_export_int_42")]
pub fn wxf_export_int_42() -> NumericArray<u8> {
    let value = 42i64;
    let bytes = wolfram_library_link::wxf_poly::to_wxf_bytes(&value).unwrap_or_default();
    NumericArray::from_slice(bytes.as_slice())
}
