//! Polymorphic WXF serializer (spec subset): encode common Rust types directly
//! using the same token scheme implemented by `wxf::ser`.
//!
//! Supported tokens (subset): integers (C/j/i/L), reals (r), strings (S), symbols (s),
//! functions (f) for `List` and `Complex`, associations (A + - rules), big integers (I).
//! Booleans and None are emitted as symbols `True`, `False`, `None`. Option::None emits symbol `None`.
//!
//! Unsupported here: DateObject, PackedArray/NumericArray, delayed rules (:), big reals (R).
//!
//! Decoding: use `wxf::de::from_wxf_bytes` into `Expr`.

use std::io::{self, Write};
use num_bigint::{BigInt, BigUint};
use crate::wxf::ser::{
    write_header, write_varint, write_integer, write_real, write_string, write_symbol,
    write_bigint, write_biguint
};

/// Newtype representing a Wolfram Symbol name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolName(pub String);
impl From<&str> for SymbolName { fn from(s: &str) -> Self { SymbolName(s.to_string()) } }
impl From<String> for SymbolName { fn from(s: String) -> Self { SymbolName(s) } }

/// Complex number encoded as tag 0x09 (two f64 little-endian parts).
#[derive(Debug, Clone, Copy, PartialEq)]
/// Simple complex number container (prototype) serialized as tag 0x09.
pub struct Complex64 { 
    /// Real component.
    pub re: f64, 
    /// Imaginary component.
    pub im: f64 
}
impl Complex64 { 
    /// Construct a new `Complex64` value.
    pub fn new(re: f64, im: f64) -> Self { Self { re, im } } 
}

/// Trait for types that can be encoded into spec‑subset WXF bytes.
pub trait WxfEncode {
    /// Write `self` to the given writer using experimental WXF tag encoding.
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()>;
}

/// Top-level convenience: write header + value.
pub fn to_wxf_bytes<T: WxfEncode>(value: &T) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    write_header(&mut buf)?; // spec header 8:
    value.encode_wxf(&mut buf)?;
    Ok(buf)
}

//============================
// Primitive Implementations
//============================
macro_rules! impl_int {
    ($($t:ty),*) => { $(
        impl WxfEncode for $t {
            fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_integer(w, *self as i64) }
        }
    )* }
}
impl_int!(i64, i32, i16, i8, u64, u32, u16, u8);

impl WxfEncode for f64 { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_real(w, *self) } }
impl WxfEncode for f32 { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_real(w, *self as f64) } }
impl WxfEncode for bool { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_symbol(w, if *self {"True"} else {"False"}) } }
impl WxfEncode for () {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
        // Emit empty List: f + 0 args + List head
        w.write_all(&[b'f'])?;
        write_varint(w, 0)?;
        write_symbol(w, "List")
    }
}
impl WxfEncode for Complex64 { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
    // f + 2 args, head Complex
    w.write_all(&[b'f'])?; write_varint(w, 2)?; write_symbol(w, "Complex")?; write_real(w, self.re)?; write_real(w, self.im)
} }
impl WxfEncode for BigInt { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_bigint(w, self) } }
impl WxfEncode for BigUint { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_biguint(w, self) } }

//============================
// Strings & Symbols
//============================
impl WxfEncode for String {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { self.as_str().encode_wxf(w) }
}
impl WxfEncode for &String {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { self.as_str().encode_wxf(w) }
}
impl WxfEncode for &str { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_string(w, self) } }
impl WxfEncode for SymbolName { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { write_symbol(w, &self.0) } }

//============================
// Collections: List & Assoc
//============================
impl<T: WxfEncode> WxfEncode for Vec<T> { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
    w.write_all(&[b'f'])?; write_varint(w, self.len() as u64)?; write_symbol(w, "List")?; for item in self { item.encode_wxf(w)?; } Ok(()) }
}
impl<T: WxfEncode> WxfEncode for &[T] { fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
    w.write_all(&[b'f'])?; write_varint(w, self.len() as u64)?; write_symbol(w, "List")?; for item in *self { item.encode_wxf(w)?; } Ok(()) }
}
/// Newtype wrapper for a Wolfram Association represented as a vector of key/value pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct Assoc<K,V>(pub Vec<(K,V)>);
impl<K,V> From<Vec<(K,V)>> for Assoc<K,V> { fn from(v: Vec<(K,V)>) -> Self { Assoc(v) } }
impl<K: WxfEncode, V: WxfEncode> WxfEncode for Assoc<K,V> {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&[b'A'])?; write_varint(w, self.0.len() as u64)?;
        for (k,v) in &self.0 { w.write_all(&[b'-'])?; k.encode_wxf(w)?; v.encode_wxf(w)?; }
        Ok(())
    }
}
impl<K: WxfEncode, V: WxfEncode> WxfEncode for &Assoc<K,V> {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { (*self).encode_wxf(w) }
}

/// Implement `WxfEncode` for the experimental `Expr` enum so it can participate
/// in polymorphic serialization chains (e.g., `Option<Expr>`, `Vec<Expr>`).
impl WxfEncode for crate::wxf::Expr {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> {
        use crate::wxf::Expr as E;
        match self {
            E::Integer(i) => write_integer(w, *i),
            E::Real(r) => write_real(w, *r),
            E::String(s) => write_string(w, s),
            E::Symbol(s) => write_symbol(w, s),
            E::List(items) => {
                w.write_all(&[b'f'])?; write_varint(w, items.len() as u64)?; write_symbol(w, "List")?;
                for item in items { item.encode_wxf(w)?; }
                Ok(())
            },
            E::Assoc(pairs) => {
                w.write_all(&[b'A'])?; write_varint(w, pairs.len() as u64)?;
                for (k,v) in pairs { w.write_all(&[b'-'])?; k.encode_wxf(w)?; v.encode_wxf(w)?; }
                Ok(())
            },
            E::Boolean(b) => write_symbol(w, if *b {"True"} else {"False"}),
            E::None => write_symbol(w, "None"),
            E::Complex(re, im) => { w.write_all(&[b'f'])?; write_varint(w, 2)?; write_symbol(w, "Complex")?; write_real(w, *re)?; write_real(w, *im) },
            E::PackedArray(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "PackedArray unsupported")),
            E::Date(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Date unsupported")),
            E::BigInt(bi) => write_bigint(w, bi),
            E::BigUint(bu) => write_biguint(w, bu),
            E::Function(head, args) => {
                w.write_all(&[b'f'])?; write_varint(w, args.len() as u64)?; head.encode_wxf(w)?; for a in args { a.encode_wxf(w)?; } Ok(())
            }
        }
    }
}

//============================
// Option<T> -> None symbol if None
//============================
impl<T: WxfEncode> WxfEncode for Option<T> {
    fn encode_wxf<W: Write>(&self, w: &mut W) -> io::Result<()> { match self { Some(v) => v.encode_wxf(w), None => write_symbol(w, "None"), } }
}

//============================
// Tuples -> function(List head) convenience (arity 2..6)
impl<A: WxfEncode, B: WxfEncode> WxfEncode for (A,B) { fn encode_wxf<W: Write>(&self, w:&mut W)->io::Result<()> { w.write_all(&[b'f'])?; write_varint(w,2)?; write_symbol(w,"List")?; self.0.encode_wxf(w)?; self.1.encode_wxf(w) } }
impl<A: WxfEncode, B: WxfEncode, C: WxfEncode> WxfEncode for (A,B,C) { fn encode_wxf<W: Write>(&self, w:&mut W)->io::Result<()> { w.write_all(&[b'f'])?; write_varint(w,3)?; write_symbol(w,"List")?; self.0.encode_wxf(w)?; self.1.encode_wxf(w)?; self.2.encode_wxf(w) } }
impl<A: WxfEncode, B: WxfEncode, C: WxfEncode, D: WxfEncode> WxfEncode for (A,B,C,D) { fn encode_wxf<W: Write>(&self, w:&mut W)->io::Result<()> { w.write_all(&[b'f'])?; write_varint(w,4)?; write_symbol(w,"List")?; self.0.encode_wxf(w)?; self.1.encode_wxf(w)?; self.2.encode_wxf(w)?; self.3.encode_wxf(w) } }
impl<A: WxfEncode, B: WxfEncode, C: WxfEncode, D: WxfEncode, E: WxfEncode> WxfEncode for (A,B,C,D,E) { fn encode_wxf<W: Write>(&self, w:&mut W)->io::Result<()> { w.write_all(&[b'f'])?; write_varint(w,5)?; write_symbol(w,"List")?; self.0.encode_wxf(w)?; self.1.encode_wxf(w)?; self.2.encode_wxf(w)?; self.3.encode_wxf(w)?; self.4.encode_wxf(w) } }
impl<A: WxfEncode, B: WxfEncode, C: WxfEncode, D: WxfEncode, E: WxfEncode, F: WxfEncode> WxfEncode for (A,B,C,D,E,F) { fn encode_wxf<W: Write>(&self, w:&mut W)->io::Result<()> { w.write_all(&[b'f'])?; write_varint(w,6)?; write_symbol(w,"List")?; self.0.encode_wxf(w)?; self.1.encode_wxf(w)?; self.2.encode_wxf(w)?; self.3.encode_wxf(w)?; self.4.encode_wxf(w)?; self.5.encode_wxf(w) } }

//============================
// Tests (only compiled under `cfg(test)`)
//============================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wxf::{de, Expr};

    #[test]
    fn encode_i64_list() {
        let data = vec![1i64,2,3,4,5];
        let bytes = to_wxf_bytes(&data).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::List(items) => assert_eq!(items.len(),5), _ => panic!("not a list") }
    }

    #[test]
    fn encode_nested_vec() {
        let nested = vec![vec![1i64,2i64], vec![3i64]];
        let bytes = to_wxf_bytes(&nested).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::List(items) => assert_eq!(items.len(),2), _ => panic!("not list") }
    }

    #[test]
    fn encode_assoc() {
        let assoc = Assoc(vec![(SymbolName::from("a"), 1i64), (SymbolName::from("b"), 2i64)]);
        let bytes = to_wxf_bytes(&assoc).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::Assoc(pairs) => assert_eq!(pairs.len(),2), _ => panic!("not assoc") }
    }

    #[test]
    fn encode_tuple_pair_as_list() {
        let pair = (42u64, BigUint::from(100u32));
        let bytes = to_wxf_bytes(&pair).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::List(items) => assert_eq!(items.len(),2), _ => panic!("not list") }
    }

    #[test]
    fn encode_bigint() {
        let bi = BigInt::parse_bytes(b"123456789012345678901234567890",10).unwrap();
        let bytes = to_wxf_bytes(&bi).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::BigInt(x) => assert_eq!(x, bi), _ => panic!("not bigint") }
    }

    #[test]
    fn encode_complex() {
        let c = Complex64::new(1.25, -3.5);
        let bytes = to_wxf_bytes(&c).unwrap();
        let expr = de::from_wxf_bytes(&bytes).unwrap();
        match expr { Expr::Complex(re, im) => { assert_eq!(re, c.re); assert_eq!(im, c.im); }, _ => panic!("not complex") }
    }
}
