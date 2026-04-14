//! WXF (Wolfram Exchange Format) codec — thin re-export of [`wolfram_expr::wxf`].
//!
//! The codec operates directly on [`wolfram_expr::Expr`], the same type used by
//! the rest of this crate and by `wstp`. There is no duplicate `Expr` enum —
//! one `Expr` serves LibraryLink, WSTP, and WXF.
//!
//! # Example
//!
//! ```no_run
//! use wolfram_library_link::expr::Expr;
//! use wolfram_library_link::wxf::{to_wxf_bytes, from_wxf_bytes};
//!
//! let e = Expr::list(vec![Expr::from(1), Expr::from(2)]);
//! let bytes = to_wxf_bytes(&e).unwrap();
//! assert_eq!(from_wxf_bytes(&bytes).unwrap(), e);
//! ```
//!
//! See [`wolfram_expr::wxf`] for the full API, including compressed (`8C:`)
//! streams and packed/numeric array handling.

pub use wolfram_expr::wxf::{
    from_wxf_bytes, to_wxf_bytes, to_wxf_bytes_compressed, ArrayElementType,
};
