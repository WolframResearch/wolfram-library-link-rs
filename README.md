# wl-library-link

This library offers bindings to Rust code from the Wolfram language.

This library is used for writing Rust programs which can be loaded by the Wolfram language
LibraryLink family of functions, specifically by
[`LibraryFunctionLoad[]`][library-function-load].

Features:

    * Call Rust functions from the Wolfram language.
    * Pass general Wolfram language expressions to and from Rust code.
    * Evaluating Wolfram expressions from Rust code.

## Usage

First, ensure that `cargo` will build a dynamic library when this crate is compiled. This
is done by setting the `crate-type` in Cargo.toml.

Next, add the `wl-expr` and `wl-library-link` crates to your dependencies in the
`Cargo.toml` file. `wl-expr` provides the type `Expr`, which is a simple Rust
representation of a Wolfram expression.

A sample Cargo.toml is:

```toml
# Cargo.toml

[package]
# This can be whatever you like, as long as it's a valid crate identifier.
name = "my-package"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]
# crate-type = ["rlib", "cdyib"]

[dependencies]
wl-expr            = { git = "ssh://github.com/ConnorGray/wl-expr.git" }
wl-library-link    = { git = "ssh://github.com/ConnorGray/wl-library-link.git" }
```

See [[ TODO ]] for a complete description of the Cargo.toml file.

Ne

```rust
use wl_expr::Expr;
use wl_library_link::generate_wrapper;


```


[library-function-load]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html