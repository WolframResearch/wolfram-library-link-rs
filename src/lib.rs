//! Automatically generated bindings to the Wolfram LibraryLink C API.
//!
//!
//! TODO: Stop generating these bindings at build time (through build.rs) and instead do
//!       something like what the `llvm-sys` crate, where the bindings are generated
//!       manually and included in the source of the crate. This means dependencies won't
//!       have to set the environment variable pointer to WolframLibrary.h. Then, have the
//!       crate version track the LibraryLink version it is associated with. W.r.t
//!       tracking the version numbers, the same should likely be done for
//!       wl-library-link (the safe wrappers).

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
)]
// The name of this file comes from `build.rs`.
include!(concat!(env!("OUT_DIR"), "/LibraryLink_bindings.rs"));