//! Tests for BigUint (unsigned arbitrary precision integer) WXF roundtrip.
use wolfram_library_link::wxf::{ser, de, Expr};
use num_bigint::BigUint;

#[test]
fn biguint_large_roundtrip() {
    let val = BigUint::parse_bytes(b"1234567890123456789012345678901234567890", 10).unwrap();
    let expr = Expr::BigUint(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let decoded = de::from_wxf_bytes(&bytes).expect("deserialize");
    assert_eq!(decoded, expr);
}

#[test]
fn biguint_zero_roundtrip() {
    let val = BigUint::from(0u8);
    let expr = Expr::BigUint(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let decoded = de::from_wxf_bytes(&bytes).expect("deserialize");
    assert_eq!(decoded, expr);
}

#[test]
fn biguint_canonical_encoding() {
    let val = BigUint::from(0xFFu32); // magnitude should be single byte 0xFF
    let expr = Expr::BigUint(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let pos = bytes.iter().position(|b| *b == 0x0D).expect("find biguint tag");
    // Structure: [0x0D][len u32][mag]
    let len_bytes = &bytes[pos + 1 .. pos + 5];
    let mag_len = u32::from_le_bytes(len_bytes.try_into().unwrap());
    assert_eq!(mag_len, 1, "expected single byte magnitude");
    let mag = bytes[pos + 5];
    assert_eq!(mag, 0xFF);
}

#[test]
fn biguint_polymorphic_encode() {
    use wolfram_library_link::wxf_poly::to_wxf_bytes;
    let val = BigUint::parse_bytes(b"98765432101234567890",10).unwrap();
    let bytes = to_wxf_bytes(&val).unwrap();
    let decoded = de::from_wxf_bytes(&bytes).unwrap();
    match decoded { Expr::BigUint(bu) => assert_eq!(bu, val), _ => panic!("not BigUint") }
}
