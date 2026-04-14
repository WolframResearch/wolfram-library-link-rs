//! Exhaustive round-trip exports for every `FromArg`/`IntoArg`/`DataStoreAdd`
//! type `wolfram-library-link` supports. Paired with
//! `RustLink/Tests/TypeConversions.wlt`.
//!
//! Each function is the simplest non-trivial round-trip — enough to
//! distinguish "value was passed faithfully" from "value was dropped or
//! coerced." The tests on the WL side check:
//!
//! - Scalars: bool, char, f32/f64 (ignored for cases below mint/mreal
//!   duplicates — see note in [`test_native_args`]).
//! - Complex (`sys::mcomplex`).
//! - Option<T> arguments (backed by DataStore).
//! - Vec<T> arguments / returns (backed by DataStore).
//! - Dates via the chrono bridge in `wolfram_expr`: round-trip through a
//!   WXF-encoded `DateObject` ByteArray. See the `date_*` functions below.

use std::convert::TryFrom;

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use wolfram_library_link::{
    self as wll,
    sys::{self, mcomplex},
    DataStore, NumericArray,
};
use wolfram_expr::{wxf, Expr};

//======================================
// Primitive scalar round-trips
//======================================

#[wll::export]
fn test_bool_not(b: bool) -> bool {
    !b
}

#[wll::export]
fn test_char_code(c: char) -> u32 {
    c as u32
}

#[wll::export]
fn test_mcomplex_conj(c: mcomplex) -> mcomplex {
    mcomplex {
        ri: [c.ri[0], -c.ri[1]],
    }
}

//======================================
// Option<T> via DataStore
//======================================

#[wll::export]
fn test_option_int_or_negone(opt: Option<i64>) -> i64 {
    opt.unwrap_or(-1)
}

#[wll::export]
fn test_option_string_echo_or_empty(opt: Option<String>) -> String {
    opt.unwrap_or_default()
}

//======================================
// Vec<T> argument types (DataStore-backed)
//======================================

#[wll::export]
fn test_vec_i32_sum(v: Vec<i32>) -> i64 {
    v.iter().map(|&x| x as i64).sum()
}

#[wll::export]
fn test_vec_string_lengths(v: Vec<String>) -> Vec<i32> {
    v.iter().map(|s| s.len() as i32).collect()
}

#[wll::export]
fn test_vec_i64_max(v: Vec<i64>) -> i64 {
    v.iter().copied().max().unwrap_or(0)
}

#[wll::export]
fn test_vec_f64_sum(v: Vec<f64>) -> f64 {
    v.iter().sum()
}

//======================================
// DataStore value add — every type
//======================================

/// Builds a DataStore with one entry per supported primitive type, for
/// end-to-end verification of `DataStoreAdd` impls.
#[wll::export]
fn test_datastore_every_type() -> DataStore {
    let mut ds = DataStore::new();
    ds.add_bool(true);
    ds.add_i64(-42_i64);
    ds.add_f64(3.25_f64);
    ds.add_str("hello");
    // One of each width so the WL side can inspect them.
    ds
}

//======================================
// NumericArray round-trips for every element type
//======================================

#[wll::export]
fn test_na_u8_identity(a: &NumericArray<u8>) -> NumericArray<u8> {
    NumericArray::from_slice(a.as_slice())
}

#[wll::export]
fn test_na_u16_identity(a: &NumericArray<u16>) -> NumericArray<u16> {
    NumericArray::from_slice(a.as_slice())
}

#[wll::export]
fn test_na_f64_negate(a: &NumericArray<f64>) -> NumericArray<f64> {
    NumericArray::from_slice(&a.as_slice().iter().map(|v| -v).collect::<Vec<_>>())
}

//======================================
// Chrono date bridge (via WXF ByteArray)
//======================================
//
// LibraryLink doesn't carry `DateObject` through an MArgument, so the
// accepted pattern is: accept WXF bytes (ByteArray), decode into an
// `Expr`, `TryFrom` into `chrono::NaiveDate` / `DateTime<Utc>`, then on
// output re-encode via `Into<Expr>` and return ByteArray.

/// Round-trip a `NaiveDate` through the chrono bridge: WL sends
/// `BinarySerialize[DateObject[{y,m,d}]]` bytes; Rust adds `days_offset`
/// days and returns the result as WXF bytes.
#[wll::export]
fn test_date_add_days(bytes: &NumericArray<u8>, days_offset: i64) -> NumericArray<u8> {
    let expr = wxf::from_wxf_bytes(bytes.as_slice()).expect("decode date bytes");
    let d: NaiveDate = (&expr)
        .try_into()
        .expect("expr was not a valid DateObject[{y,m,d}]");
    let shifted = d
        .checked_add_signed(chrono::Duration::days(days_offset))
        .expect("date arithmetic overflow");
    let out_expr: Expr = shifted.into();
    NumericArray::from_slice(&wxf::to_wxf_bytes(&out_expr).unwrap())
}

/// Round-trip a `DateTime<Utc>` through the chrono bridge: Rust adds
/// `seconds_offset` seconds and returns the result.
#[wll::export]
fn test_datetime_add_seconds(
    bytes: &NumericArray<u8>,
    seconds_offset: i64,
) -> NumericArray<u8> {
    let expr = wxf::from_wxf_bytes(bytes.as_slice()).expect("decode datetime bytes");
    let dt: DateTime<Utc> = DateTime::<Utc>::try_from(&expr)
        .expect("expr was not a valid DateObject[{y,m,d,h,m,s}, \"Instant\", ...]");
    let shifted = dt + chrono::Duration::seconds(seconds_offset);
    let out_expr: Expr = shifted.into();
    NumericArray::from_slice(&wxf::to_wxf_bytes(&out_expr).unwrap())
}

/// Construct a `DateTime<Utc>` from primitive components and return it as
/// a WXF-encoded `DateObject`. Exercises the Rust → WL direction.
#[wll::export]
fn test_build_datetime(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
) -> NumericArray<u8> {
    let dt = Utc
        .with_ymd_and_hms(
            year as i32,
            month as u32,
            day as u32,
            hour as u32,
            minute as u32,
            second as u32,
        )
        .single()
        .expect("invalid datetime components");
    let expr: Expr = dt.into();
    NumericArray::from_slice(&wxf::to_wxf_bytes(&expr).unwrap())
}

//======================================
// num_complex bridge (Complex<f64> <-> Expr via WXF ByteArray)
//======================================

/// Round-trip a `Complex<f64>` through the bridge: Rust multiplies by `i`
/// (rotates 90°) and returns WXF bytes.
#[wll::export]
fn test_complex_rotate_90(bytes: &NumericArray<u8>) -> NumericArray<u8> {
    use num_complex::Complex;
    let expr = wxf::from_wxf_bytes(bytes.as_slice()).expect("decode");
    let c: Complex<f64> = (&expr).try_into().expect("not a Complex[re,im]");
    let rotated = c * Complex::new(0.0_f64, 1.0);
    let out: Expr = rotated.into();
    NumericArray::from_slice(&wxf::to_wxf_bytes(&out).unwrap())
}

//======================================
// serde_json bridge (JSON string <-> Expr via WXF ByteArray)
//======================================

/// Accept a JSON string, convert via `serde_json::Value` → `Expr`,
/// WXF-encode, return bytes for WL-side `BinaryDeserialize`.
#[wll::export]
fn test_json_to_wxf(json: String) -> NumericArray<u8> {
    let v: serde_json::Value = serde_json::from_str(&json).expect("invalid JSON");
    let expr: Expr = (&v).into();
    NumericArray::from_slice(&wxf::to_wxf_bytes(&expr).unwrap())
}

/// Inverse: accept WXF bytes, decode to `Expr`, convert through the
/// lossy bridge to a JSON string.
#[wll::export]
fn test_wxf_to_json(bytes: &NumericArray<u8>) -> String {
    let expr = wxf::from_wxf_bytes(bytes.as_slice()).expect("decode");
    let v = wolfram_expr::expr_to_json(&expr).expect("not json-representable");
    serde_json::to_string(&v).expect("serialize json")
}

/// Native lossless path: serialize an `Expr` via the `Serialize` impl
/// and return the JSON string. WL can `ImportString[..., "RawJSON"]`
/// and inspect the externally-tagged structure.
#[wll::export]
fn test_expr_serde_json(bytes: &NumericArray<u8>) -> String {
    let expr = wxf::from_wxf_bytes(bytes.as_slice()).expect("decode");
    serde_json::to_string(&expr).expect("serialize")
}

//======================================
// Silence unused-import warnings when some type paths end up dead.
//======================================

#[allow(dead_code)]
fn _unused_sys_ref(_: sys::mint) {}
