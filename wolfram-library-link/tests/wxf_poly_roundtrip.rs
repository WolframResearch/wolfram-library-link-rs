//! Roundtrip tests using the polymorphic WXF serializer (`wxf_poly`).
//! These validate that native Rust types encoded via `WxfEncode` decode back
//! to the expected `Expr` structure using the enum-based deserializer.

use wolfram_library_link::wxf::{de, Expr};
use wolfram_library_link::wxf_poly::{self, SymbolName, Complex64};
use num_bigint::BigInt;

fn decode(bytes: Vec<u8>) -> Expr { de::from_wxf_bytes(&bytes).expect("decode") }

#[test]
fn roundtrip_integer() {
    let bytes = wxf_poly::to_wxf_bytes(&123i64).unwrap();
    assert_eq!(decode(bytes), Expr::Integer(123));
}

#[test]
fn roundtrip_float() {
    let bytes = wxf_poly::to_wxf_bytes(&3.5f64).unwrap();
    match decode(bytes) { Expr::Real(r) => assert_eq!(r, 3.5), _ => panic!() }
}

#[test]
fn roundtrip_string() {
    let s = "hello poly";
    let bytes = wxf_poly::to_wxf_bytes(&s).unwrap();
    assert_eq!(decode(bytes), Expr::String(s.to_string()));
}

#[test]
fn roundtrip_symbol() {
    let sym = SymbolName::from("System`Pi");
    let bytes = wxf_poly::to_wxf_bytes(&sym).unwrap();
    assert_eq!(decode(bytes), Expr::Symbol("System`Pi".into()));
}

#[test]
fn roundtrip_list_nested() {
    let data = vec![1i64,2i64,3i64];
    let outer = vec![data.clone(), vec![4i64]]; // Vec<Vec<i64>>
    let bytes = wxf_poly::to_wxf_bytes(&outer).unwrap();
    let expr = decode(bytes);
    match expr { Expr::List(items) => {
        assert_eq!(items.len(),2);
        match &items[0] { Expr::List(inner) => assert_eq!(inner.len(),3), _ => panic!() }
    }, _ => panic!() }
}

#[test]
fn roundtrip_association() {
    let assoc = vec![(SymbolName::from("a"), 1i64), (SymbolName::from("b"), 2i64)];
    let bytes = wxf_poly::to_wxf_bytes(&assoc).unwrap();
    let expr = decode(bytes);
    match expr { Expr::Assoc(pairs) => {
        assert_eq!(pairs.len(),2);
        assert!(matches!(pairs[0].0, Expr::Symbol(_)));
    }, _ => panic!() }
}

#[test]
fn roundtrip_bool_null() {
    let bbytes = wxf_poly::to_wxf_bytes(&true).unwrap();
    assert_eq!(decode(bbytes), Expr::Boolean(true));
    let nbytes = wxf_poly::to_wxf_bytes(&()).unwrap();
    assert_eq!(decode(nbytes), Expr::None);
}

#[test]
fn roundtrip_complex() {
    let c = Complex64::new(1.25,-9.0);
    let bytes = wxf_poly::to_wxf_bytes(&c).unwrap();
    match decode(bytes) { Expr::Complex(re, im) => { assert_eq!(re,1.25); assert_eq!(im,-9.0); }, _ => panic!() }
}

#[test]
fn roundtrip_bigint() {
    let bi = BigInt::parse_bytes(b"987654321012345678909876543210",10).unwrap();
    let bytes = wxf_poly::to_wxf_bytes(&bi).unwrap();
    match decode(bytes) { Expr::BigInt(x) => assert_eq!(x, bi), _ => panic!() }
}

#[test]
fn roundtrip_option_some_none() {
    let some_bytes = wxf_poly::to_wxf_bytes(&Some(5i64)).unwrap();
    assert_eq!(decode(some_bytes), Expr::Integer(5));
    let none_bytes = wxf_poly::to_wxf_bytes(&Option::<i64>::None).unwrap();
    assert_eq!(decode(none_bytes), Expr::None);
}

#[test]
fn deterministic_serialization() {
    let value = vec![SymbolName::from("x"), SymbolName::from("y")];
    let a = wxf_poly::to_wxf_bytes(&value).unwrap();
    let b = wxf_poly::to_wxf_bytes(&value).unwrap();
    assert_eq!(a, b, "bytes differ for identical value");
}
