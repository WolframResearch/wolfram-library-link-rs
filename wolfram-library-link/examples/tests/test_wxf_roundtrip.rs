//! Test library functions for WXF native roundtrip via ByteArray.
//! Accepts ByteArray (NumericArray<u8>), deserializes in WL, re-serializes, returns bytes.

use wolfram_library_link::wxf::{de, ser};
use wolfram_library_link::wxf_poly::to_wxf_bytes;
use wolfram_library_link::{export, NumericArray};
use num_bigint::BigUint;

/// Identity roundtrip using WL BinaryDeserialize/BinarySerialize.
/// Input: ByteArray (UBit8 NumericArray) containing WXF bytes.
/// Output: ByteArray with the same WXF bytes after WL roundtrip.
#[export(name = "test_wxf_identity_roundtrip")]
pub fn wxf_identity_roundtrip(bytes: &NumericArray<u8>) -> NumericArray<u8> {
    // Spec subset strict roundtrip: decode then re-encode; on any error return empty array.
    let expr = de::from_wxf_bytes(bytes.as_slice()).expect("decode failed");
    let out = ser::to_wxf_bytes(&expr).expect("re-encode failed");
    NumericArray::from_slice(&out)
}


#[export]
pub fn test_string_vec(_strings: Vec<String>) -> NumericArray<u8> {
    let none: Vec<Vec<Option<BigUint>>> = vec![vec![None]];
    let out = to_wxf_bytes(&none).unwrap_or_default();
    NumericArray::from_slice(&out)
}