//! Tests for Option serialization using both enum convenience and polymorphic trait.
use wolfram_library_link::wxf::{ser, de, Expr};
use wolfram_library_link::wxf_poly; // WxfEncode trait used implicitly

#[test]
fn option_none_serializes_to_null() {
    let bytes = ser::to_wxf_bytes_option(None).unwrap();
    let decoded = de::from_wxf_bytes(&bytes).unwrap();
    assert!(matches!(decoded, Expr::Null));
}

#[test]
fn option_some_expr() {
    let expr = Expr::integer(42);
    let bytes = ser::to_wxf_bytes_option(Some(&expr)).unwrap();
    let decoded = de::from_wxf_bytes(&bytes).unwrap();
    assert_eq!(decoded, expr);
}

#[test]
fn polymorphic_option_expr() {
    let expr = Expr::string("hello");
    let some_bytes = wxf_poly::to_wxf_bytes(&Some(expr.clone())).unwrap();
    let some_decoded = de::from_wxf_bytes(&some_bytes).unwrap();
    assert_eq!(some_decoded, expr);
    let none_bytes = wxf_poly::to_wxf_bytes(&Option::<Expr>::None).unwrap();
    let none_decoded = de::from_wxf_bytes(&none_bytes).unwrap();
    assert!(matches!(none_decoded, Expr::Null));
}
