//! Spec‑compliant (subset) WXF (Wolfram Expression Format) serialization/deserialization
//! for a simplified Rust representation of Wolfram Language expressions.
//!
//! This replaces the earlier experimental custom tag format with tokens and layout
//! matching the official WXF description for the following constructs:
//! * Machine integers (`C`,`j`,`i`,`L`)
//! * Machine reals (`r`)
//! * Strings (`S`)
//! * Symbols (`s`)
//! * Functions (`f`) used for `List` and `Complex`
//! * Associations (`A`) with rules (`-`) (delayed rules `:` not yet supported)
//! * Big integers (`I`) and big unsigned integers encoded as decimal digit sequences
//!
//! Unsupported / TODO:
//! * DateObject (needs faithful head + internal structure)
//! * PackedArray / NumericArray tokens (`\xC1`, `\xC2`) – previous opaque impl removed
//! * Big reals (`R`)
//! * Delayed rules (`:`)
//! * Compression header variant `8C:`
//!
//! Header: we emit the uncompressed header `8:` (ASCII) exactly.
//! Lengths and counts use WXF varint encoding (7 bits payload per byte, MSB continuation).
//!
//! References: https://reference.wolfram.com/language/tutorial/WXFFormatDescription.html

/// Simplified Wolfram expression representation used by the experimental WXF encoder.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Machine‐sized signed integer (`Integer`).
    Integer(i64),
    /// IEEE 754 double precision floating point (`Real`).
    Real(f64),
    /// UTF‑8 string data (`String`).
    String(String),
    /// Symbol name (e.g. `"System`Pi"`). Stored as UTF‑8.
    Symbol(String),
    /// Ordered sequence (`List`).
    List(Vec<Expr>),
    /// Association key/value pairs (`Association`).
    Assoc(Vec<(Expr, Expr)>),
    /// Boolean truth value (`True` / `False`).
    Boolean(bool),
    /// The Wolfram `None` object.
    None,
    /// Complex number with real & imaginary parts (`Complex[re, im]`).
    Complex(f64, f64),
    /// Packed numeric array placeholder (token not currently emitted). Retained for
    /// backwards compatibility; serialization returns an error if encountered.
    PackedArray(PackedArray),
    /// DateTime components (placeholder; currently encoded as symbol `DateObject` is NOT
    /// faithful – decoding from WL bytes not implemented). Serializing returns error.
    Date(Date),
    /// Arbitrary precision integer (`BigInteger`).
    BigInt(num_bigint::BigInt),
    /// Unsigned arbitrary precision integer (`BigUnsignedInteger`).
    BigUint(num_bigint::BigUint),
    /// Generic function: head expression + argument expressions (for arbitrary WL heads not
    /// specially recognized like `List` or `Complex`).
    Function(Box<Expr>, Vec<Expr>),
}

/// Opaque packed numeric array payload.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedArray {
    /// Element data type.
    pub dtype: PackedArrayType,
    /// Dimensions (row‑major ordering implied).
    pub dims: Vec<usize>,
    /// Raw contiguous element bytes (little‑endian for multi‑byte types).
    pub data: Vec<u8>,
}

/// Supported packed array element kinds. (Prototype subset.)
#[derive(Debug, Clone, PartialEq)]
pub enum PackedArrayType {
    /// 8‑bit signed integer.
    Int8,
    /// 16‑bit signed integer (little‑endian).
    Int16,
    /// 32‑bit signed integer (little‑endian).
    Int32,
    /// 64‑bit signed integer (little‑endian).
    Int64,
    /// 8‑bit unsigned integer.
    UInt8,
    /// 16‑bit unsigned integer.
    UInt16,
    /// 32‑bit unsigned integer.
    UInt32,
    /// 64‑bit unsigned integer.
    UInt64,
    /// 32‑bit IEEE float.
    Float32,
    /// 64‑bit IEEE float.
    Float64,
}

/// Broken‑down date/time components (UTC implied, no leap second handling).
#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    /// Full year, e.g. 2025.
    pub year: i32,
    /// Month 1–12.
    pub month: u8,
    /// Day 1–31.
    pub day: u8,
    /// Hour 0–23.
    pub hour: u8,
    /// Minute 0–59.
    pub minute: u8,
    /// Second 0–59 (no leap second support).
    pub second: u8,
    /// Milliseconds 0–999.
    pub ms: u16,
}

impl Expr {
    /// Construct a `Symbol` expression.
    pub fn symbol<S: Into<String>>(s: S) -> Self {
        Expr::Symbol(s.into())
    }
    /// Construct an `Integer` expression.
    pub fn integer(i: i64) -> Self {
        Expr::Integer(i)
    }
    /// Construct a `Real` expression.
    pub fn real(r: f64) -> Self {
        Expr::Real(r)
    }
    /// Construct a `String` expression.
    pub fn string<S: Into<String>>(s: S) -> Self {
        Expr::String(s.into())
    }
    /// Construct a `List` expression from any `Into<Vec<Expr>>`.
    pub fn list<L: Into<Vec<Expr>>>(l: L) -> Self {
        Expr::List(l.into())
    }
    /// Construct an `Assoc` (Association) expression.
    pub fn assoc<A: Into<Vec<(Expr, Expr)>>>(a: A) -> Self {
        Expr::Assoc(a.into())
    }
}
/// Serialization routines (Expr -> WXF bytes).
pub mod ser {
    use super::Expr;
    use std::io::{self, Write};
    /// Encode an `Expr` into experimental WXF bytes.
    ///
    /// Returns a newly allocated `Vec<u8>` containing header + body.
    pub fn to_wxf_bytes(expr: &Expr) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        write_header(&mut buf)?;
        write_expr(&mut buf, expr)?;
        Ok(buf)
    }

    /// Convenience: encode an `Option<&Expr>`, mapping `None` to the `None` tag.
    pub fn to_wxf_bytes_option(expr: Option<&Expr>) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        write_header(&mut buf)?;
        match expr {
            Some(e) => write_expr(&mut buf, e)?,
            None => write_symbol(&mut buf, "None")?,
        }
        Ok(buf)
    }

    fn write_expr<W: Write>(w: &mut W, expr: &Expr) -> io::Result<()> {
        match expr {
            Expr::Integer(i) => write_integer(w, *i),
            Expr::Real(r) => write_real(w, *r),
            Expr::String(s) => write_string(w, s),
            Expr::Symbol(s) => write_symbol(w, s),
            Expr::List(items) => write_list(w, items),
            Expr::Assoc(pairs) => write_assoc(w, pairs),
            Expr::Boolean(b) => write_symbol(w, if *b { "True" } else { "False" }),
            Expr::None => write_symbol(w, "None"),
            Expr::Complex(re, im) => write_complex(w, *re, *im),
            Expr::PackedArray(pa) => write_packed_array(w, pa),
            Expr::Date(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Date unsupported in spec subset")),
            Expr::BigInt(bi) => write_bigint(w, bi),
            Expr::BigUint(bu) => write_biguint(w, bu),
            Expr::Function(head, args) => write_function(w, head, args),
        }
    }

    // -------- Spec helper writers --------
    pub(crate) fn write_header<W: Write>(w: &mut W) -> io::Result<()> { w.write_all(b"8:") }

    pub(crate) fn write_varint<W: Write>(w: &mut W, mut v: u64) -> io::Result<()> {
        let mut buf = [0u8; 10]; // enough for 64-bit varint
        let mut i = 0;
        loop {
            let mut byte = (v & 0x7F) as u8;
            v >>= 7;
            if v != 0 { byte |= 0x80; }
            buf[i] = byte; i += 1;
            if v == 0 { break; }
        }
        w.write_all(&buf[..i])
    }

    pub(crate) fn write_integer<W: Write>(w: &mut W, i: i64) -> io::Result<()> {
        if i >= i8::MIN as i64 && i <= i8::MAX as i64 {
            w.write_all(&[b'C'])?; w.write_all(&[i as i8 as u8])
        } else if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
            w.write_all(&[b'j'])?; w.write_all(&(i as i16).to_le_bytes())
        } else if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            w.write_all(&[b'i'])?; w.write_all(&(i as i32).to_le_bytes())
        } else {
            w.write_all(&[b'L'])?; w.write_all(&i.to_le_bytes())
        }
    }
    pub(crate) fn write_real<W: Write>(w: &mut W, r: f64) -> io::Result<()> { w.write_all(&[b'r'])?; w.write_all(&r.to_le_bytes()) }
    pub(crate) fn write_string<W: Write>(w: &mut W, s: &str) -> io::Result<()> { w.write_all(&[b'S'])?; write_varint(w, s.len() as u64)?; w.write_all(s.as_bytes()) }
    pub(crate) fn write_symbol<W: Write>(w: &mut W, s: &str) -> io::Result<()> { w.write_all(&[b's'])?; write_varint(w, s.len() as u64)?; w.write_all(s.as_bytes()) }

    pub(crate) fn write_function<W: Write>(w: &mut W, head: &Expr, args: &[Expr]) -> io::Result<()> {
        w.write_all(&[b'f'])?; write_varint(w, args.len() as u64)?; write_expr(w, head)?; for a in args { write_expr(w, a)?; } Ok(())
    }

    pub(crate) fn write_list<W: Write>(w: &mut W, items: &[Expr]) -> io::Result<()> {
        let head = Expr::Symbol("List".to_string()); write_function(w, &head, items)
    }
    pub(crate) fn write_complex<W: Write>(w: &mut W, re: f64, im: f64) -> io::Result<()> {
        let head = Expr::Symbol("Complex".to_string());
        let r1 = Expr::Real(re); let r2 = Expr::Real(im); write_function(w, &head, &[r1, r2])
    }
    pub(crate) fn write_assoc<W: Write>(w: &mut W, pairs: &[(Expr, Expr)]) -> io::Result<()> {
        w.write_all(&[b'A'])?; write_varint(w, pairs.len() as u64)?;
        for (k,v) in pairs { w.write_all(&[b'-'])?; write_expr(w, k)?; write_expr(w, v)?; }
        Ok(())
    }
    pub(crate) fn write_bigint<W: Write>(w: &mut W, bi: &num_bigint::BigInt) -> io::Result<()> {
        let s = bi.to_string(); w.write_all(&[b'I'])?; write_varint(w, s.len() as u64)?; w.write_all(s.as_bytes())
    }
    pub(crate) fn write_biguint<W: Write>(w: &mut W, bu: &num_bigint::BigUint) -> io::Result<()> {
        let s = bu.to_string(); w.write_all(&[b'I'])?; write_varint(w, s.len() as u64)?; w.write_all(s.as_bytes())
    }
    pub(crate) fn write_packed_array<W: Write>(w: &mut W, pa: &super::PackedArray) -> io::Result<()> {
        // Token 193 (Á) packed array. value type mapping subset.
        w.write_all(&[193u8])?; w.write_all(&[packed_value_type_token(&pa.dtype)])?;
        write_varint(w, pa.dims.len() as u64)?; for d in &pa.dims { write_varint(w, *d as u64)?; }
        w.write_all(&pa.data)?; Ok(())
    }

    fn packed_value_type_token(t: &super::PackedArrayType) -> u8 {
        match t {
            super::PackedArrayType::Int8 => 0,
            super::PackedArrayType::Int16 => 1,
            super::PackedArrayType::Int32 => 2,
            super::PackedArrayType::Int64 => 3,
            super::PackedArrayType::UInt8 => 16,
            super::PackedArrayType::UInt16 => 17,
            super::PackedArrayType::UInt32 => 18,
            super::PackedArrayType::UInt64 => 19,
            super::PackedArrayType::Float32 => 34,
            super::PackedArrayType::Float64 => 35,
        }
    }
}

/// --- WXF Deserializer (spec subset) ---
pub mod de {
    use super::Expr;
    use std::io::{self, Cursor, Read};

    /// Decode WXF bytes into an `Expr`.
    pub fn from_wxf_bytes(bytes: &[u8]) -> io::Result<Expr> {
        let mut cur = Cursor::new(bytes);
        read_header(&mut cur)?; read_expr(&mut cur)
    }

    fn read_header<R: Read>(r: &mut R) -> io::Result<()> {
        // Expect ASCII header ending with ':' starting with '8'.
        let mut buf = Vec::new();
        let mut one = [0u8;1];
        loop { r.read_exact(&mut one)?; if one[0] == b':' { break; } buf.push(one[0]); if buf.len()>8 { return Err(io::Error::new(io::ErrorKind::InvalidData,"Header too long")); } }
        if buf.is_empty() || buf[0] != b'8' { return Err(io::Error::new(io::ErrorKind::InvalidData,"Missing version 8")); }
        Ok(())
    }

    fn read_varint<R: Read>(r: &mut R) -> io::Result<u64> {
        let mut value=0u64; let mut shift=0u32; let mut b=[0u8;1];
        loop { r.read_exact(&mut b)?; let byte=b[0]; value |= ((byte & 0x7F) as u64) << shift; shift += 7; if byte & 0x80 == 0 { break; } if shift>=64 { return Err(io::Error::new(io::ErrorKind::InvalidData,"Varint overflow")); } }
        Ok(value)
    }

    fn read_expr<R: Read>(r: &mut R) -> io::Result<Expr> {
        let mut tag=[0u8;1]; r.read_exact(&mut tag)?; match tag[0] {
            b'C' => { let mut b=[0u8;1]; r.read_exact(&mut b)?; Ok(Expr::Integer(i8::from_le_bytes(b) as i64)) },
            b'j' => { let mut b=[0u8;2]; r.read_exact(&mut b)?; Ok(Expr::Integer(i16::from_le_bytes(b) as i64)) },
            b'i' => { let mut b=[0u8;4]; r.read_exact(&mut b)?; Ok(Expr::Integer(i32::from_le_bytes(b) as i64)) },
            b'L' => { let mut b=[0u8;8]; r.read_exact(&mut b)?; Ok(Expr::Integer(i64::from_le_bytes(b))) },
            b'r' => { let mut b=[0u8;8]; r.read_exact(&mut b)?; Ok(Expr::Real(f64::from_le_bytes(b))) },
            b'S' => { let len=read_varint(r)? as usize; let mut buf=vec![0u8;len]; r.read_exact(&mut buf)?; Ok(Expr::String(String::from_utf8(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData,"utf8"))?)) },
            b's' => { let len=read_varint(r)? as usize; let mut buf=vec![0u8;len]; r.read_exact(&mut buf)?; Ok(Expr::Symbol(String::from_utf8(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData,"utf8"))?)) },
            b'I' => { let len=read_varint(r)? as usize; let mut buf=vec![0u8;len]; r.read_exact(&mut buf)?; let s=String::from_utf8(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData,"utf8"))?; Ok(Expr::BigInt(s.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData,"bigint"))?)) },
            b'A' => { let count=read_varint(r)? as usize; let mut pairs=Vec::with_capacity(count); for _ in 0..count { let mut rule=[0u8;1]; r.read_exact(&mut rule)?; if rule[0] != b'-' { return Err(io::Error::new(io::ErrorKind::InvalidData,"expected rule")); } let k=read_expr(r)?; let v=read_expr(r)?; pairs.push((k,v)); } Ok(Expr::Assoc(pairs)) },
            b'f' => { let argc=read_varint(r)? as usize; let head=read_expr(r)?; let mut args=Vec::with_capacity(argc); for _ in 0..argc { args.push(read_expr(r)?); }
                match head { Expr::Symbol(ref s) if s=="List" => Ok(Expr::List(args)), Expr::Symbol(ref s) if s=="Complex" && args.len()==2 => { match (&args[0],&args[1]) { (Expr::Real(r1),Expr::Real(r2)) => Ok(Expr::Complex(*r1,*r2)), _ => Ok(Expr::Function(Box::new(head), args)) } }, _ => Ok(Expr::Function(Box::new(head), args)) }
            },
            193u8 => { // Packed array Á
                let mut vtype=[0u8;1]; r.read_exact(&mut vtype)?; let rank=read_varint(r)? as usize; let mut dims=Vec::with_capacity(rank); for _ in 0..rank { dims.push(read_varint(r)? as usize); }
                let elem_size = 1usize << (vtype[0] & 0x0F); let total_elems = dims.iter().product::<usize>(); let data_len = elem_size * total_elems; let mut data=vec![0u8; data_len]; r.read_exact(&mut data)?;
                let dtype = match vtype[0] { 0 => super::PackedArrayType::Int8, 1 => super::PackedArrayType::Int16, 2 => super::PackedArrayType::Int32, 3 => super::PackedArrayType::Int64, 16 => super::PackedArrayType::UInt8, 17 => super::PackedArrayType::UInt16, 18 => super::PackedArrayType::UInt32, 19 => super::PackedArrayType::UInt64, 34 => super::PackedArrayType::Float32, 35 => super::PackedArrayType::Float64, _ => return Err(io::Error::new(io::ErrorKind::InvalidData,"Unsupported packed array type")) };
                Ok(Expr::PackedArray(super::PackedArray{dtype,dims,data}))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData,"Unknown token"))
        }
    }
}

//======================================
// Conversion from wolfram_expr::Expr
//======================================

/// Convert a [`wolfram_expr::Expr`][crate::expr::Expr] to a [`wxf::Expr`][Expr].
///
/// This enables using the simpler wxf::Expr for DataStore encoding.
impl From<&crate::expr::Expr> for Expr {
    fn from(expr: &crate::expr::Expr) -> Self {
        use crate::expr::ExprKind;
        
        match expr.kind() {
            ExprKind::Integer(i) => Expr::Integer(*i),
            ExprKind::Real(r) => Expr::Real(r.into_inner()),
            ExprKind::String(s) => Expr::String(s.clone()),
            ExprKind::Symbol(sym) => Expr::Symbol(sym.as_str().to_string()),
            ExprKind::Normal(normal) => {
                let head_expr: Expr = normal.head().into();
                let args: Vec<Expr> = normal.elements().iter().map(|e| e.into()).collect();
                
                // Check for special cases
                match normal.head().kind() {
                    ExprKind::Symbol(sym) if sym.as_str() == "System`List" => {
                        Expr::List(args)
                    },
                    ExprKind::Symbol(sym) if sym.as_str() == "System`Complex" && args.len() == 2 => {
                        // Try to extract re/im as f64
                        if let (Expr::Real(re), Expr::Real(im)) = (&args[0], &args[1]) {
                            Expr::Complex(*re, *im)
                        } else if let (Expr::Integer(re), Expr::Integer(im)) = (&args[0], &args[1]) {
                            Expr::Complex(*re as f64, *im as f64)
                        } else {
                            Expr::Function(Box::new(head_expr), args)
                        }
                    },
                    ExprKind::Symbol(sym) if sym.as_str() == "System`Association" => {
                        // Try to convert rules to pairs
                        let mut pairs = Vec::new();
                        for arg in &args {
                            if let Expr::Function(head, rule_args) = arg {
                                if let Expr::Symbol(rule_sym) = head.as_ref() {
                                    if rule_sym == "System`Rule" && rule_args.len() == 2 {
                                        pairs.push((rule_args[0].clone(), rule_args[1].clone()));
                                        continue;
                                    }
                                }
                            }
                            // Not a proper association structure, fall back to function
                            return Expr::Function(Box::new(head_expr), args);
                        }
                        Expr::Assoc(pairs)
                    },
                    _ => Expr::Function(Box::new(head_expr), args),
                }
            },
        }
    }
}
