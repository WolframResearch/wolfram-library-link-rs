# wl-library-link

This library offers bindings to Rust code from the Wolfram language.

This library is used for writing Rust programs which can be loaded by the Wolfram language
LibraryLink family of functions, specifically by
[`LibraryFunctionLoad[]`][library-function-load].

Features:

  * Call Rust functions from the Wolfram language.
  * Pass general Wolfram language expressions to and from Rust code.
  * Evaluate Wolfram expressions from Rust code.
  * Check for and respond to Wolfram language aborts while in Rust code.

## Usage

First, ensure that `cargo` will build a dynamic library when this crate is compiled. This
is done by setting `crate-type = ["cdylib"]` in Cargo.toml. This library can be loaded by
[LibraryLink][library-link].

Next, add the `wl-expr` and `wl-library-link` crates to your dependencies in the
`Cargo.toml` file. `wl-expr` provides the type `Expr`, which is a simple Rust
representation of a Wolfram expression.

A correctly configured Cargo.toml looks like:

```toml
### Cargo.toml

[package]
# This can be whatever you like, as long as it's a valid crate identifier.
name = "my-package"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
wl-expr            = { git = "ssh://github.com/ConnorGray/wl-expr.git" }
wl-library-link    = { git = "ssh://github.com/ConnorGray/wl-library-link.git" }
```

See [[ TODO ]] for a complete description of the Cargo.toml file.

Next

```rust
// ### main.rs

use wl_expr::Expr;
use wl_library_link::generate_wrapper;

generate_wrapper![GET_HEAD # get_head(e: Expr) -> Expr];

// TODO: #[wl_library_link::wrap(wrapper_name = "GET_HEAD")]
fn get_normal_head(expr: Expr) -> Expr {
    match expr.kind() {
        ExprKind::Normal(normal) => normal.head.clone(),
        ExprKind::Symbol(_) | ExprKind::String(_) | ExprKind::Number(_) => wlexpr! {
            Failure["HeadOfAtomic", <|
                "Message" -> "Expected non-atomic expression"
            |>]
        }
    }
}
```

Finally, build the library by executing the following commands in the terminal:

```shell
$ cargo build
```

[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html
[library-function-load]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html

### Creating a library which is usable from Rust and Wolfram

`crate-type = ["rlib", "cdyib"]`