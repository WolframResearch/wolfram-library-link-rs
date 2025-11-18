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
    // Canonical spec-subset encoding used by current serializer: 'I' token + varint length + ASCII decimal digits.
    let val = BigInt::parse_bytes(b"255", 10).unwrap(); // fits in machine int but Expr::BigInt forces big-int path
    let expr = Expr::BigInt(val.clone());
    let bytes = ser::to_wxf_bytes(&expr).expect("serialize");
    // Header should start with 8:
    assert!(bytes.starts_with(b"8:"));
    // Find 'I' token (ASCII 0x49)
    let pos = bytes.iter().position(|b| *b == b'I').expect("find bigint I token");
    // Parse varint length immediately after token.
    let mut idx = pos + 1;
    let mut len: u64 = 0;
    let mut shift = 0u32;
    loop {
        let byte = bytes[idx];
        idx += 1;
        len |= ((byte & 0x7F) as u64) << shift;
        if byte & 0x80 == 0 { break; }
        shift += 7;
        assert!(shift < 64, "varint overflow");
    }
    assert_eq!(len, 3, "length should be number of decimal digits");
    let digits = &bytes[idx .. idx + (len as usize)];
    assert_eq!(digits, b"255");
}
