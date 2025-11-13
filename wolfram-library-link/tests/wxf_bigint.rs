//! Tests for BigInt WXF roundtrip.
use wolfram_library_link::wxf::{ser, de, Expr};
use num_bigint::BigInt;

#[test]
fn bigint_large_positive_roundtrip() {
    let val = BigInt::parse_bytes(b"123456789012345678901234567890123456789012345", 10).unwrap();
    let expr = Expr::BigInt(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let decoded = de::from_wxf_bytes(&bytes).expect("deserialize");
    assert_eq!(decoded, expr);
}

#[test]
fn bigint_large_negative_roundtrip() {
    let val = -BigInt::from(2u8).pow(300); // -2^300
    let expr = Expr::BigInt(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let decoded = de::from_wxf_bytes(&bytes).expect("deserialize");
    assert_eq!(decoded, expr);
}

#[test]
fn bigint_zero_roundtrip() {
    let val = BigInt::from(0);
    let expr = Expr::BigInt(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    let decoded = de::from_wxf_bytes(&bytes).expect("deserialize");
    assert_eq!(decoded, expr);
}

#[test]
fn bigint_canonical_encoding() {
    let val = BigInt::parse_bytes(b"255", 10).unwrap(); // 0xFF
    let expr = Expr::BigInt(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    // Scan for tag 0x0C then verify length is 1 and magnitude single byte 0xFF.
    let pos = bytes.iter().position(|b| *b == 0x0C).expect("find bigint tag");
    // Structure: [0x0C][sign][len u32][mag bytes]
    let sign = bytes[pos + 1];
    assert_eq!(sign, 1, "expected positive sign");
    let len_bytes = &bytes[pos + 2 .. pos + 6];
    let mag_len = u32::from_le_bytes(len_bytes.try_into().unwrap());
    assert_eq!(mag_len, 1, "expected single byte magnitude");
    let mag = bytes[pos + 6];
    assert_eq!(mag, 0xFF);
}
