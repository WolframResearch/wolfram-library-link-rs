# wolfram-library-link

This library offers bindings to Rust code from the Wolfram language.

This library is used for writing Rust programs that can be loaded by the Wolfram language
LibraryLink family of functions, specifically by
[`LibraryFunctionLoad[]`][library-function-load].

Features:

  * Call Rust functions from the Wolfram language.
  * Pass general Wolfram language expressions to and from Rust code.
  * Evaluate Wolfram expressions from Rust code.
  * Check for and respond to Wolfram language aborts while in Rust code.
  * Seamlessly construct native Rust datastructures from expressions using the pattern
    language via `derive(FromExpr)`.
  * Safe API for the WSTP, using the [`wstp`][wstp] crate.

Advantages over the LibraryLink/WSTP C API:

  * The API is easier to use
    - No need to worry about managing allocations
    - No potential memory unsafety
    - Easy and safe method call to evaluate an expression from Rust code
  * Ability to use a safer, but still just as performant, modern programming language

## Quick Start Guide

Create a new Rust library by running:

```shell
$ cargo new --lib my-package
```

Next, ensure that your project's `Cargo.toml` is correctly configured to be used as a
Wolfram LibraryLink library. This means:

  * Setting `crate-type = ["cdylib"]`
  * Adding `wl-expr` and `wolfram-library-link` as dependencies

By setting `crate-type` to `cdylib` we tell `cargo` to build a dynamic library, which
will be loadable using Wolfram [LibraryLink][library-link].

The `wl-expr` and `wolfram-library-link` dependencies provide, respectively, an `Expr` type,
which is a simple Rust representation of a Wolfram expression, and an API for interacting
with the Wolfram language from Rust.

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
wl-expr = { git = "ssh://github.com/ConnorGray/wl-expr.git" }
wolfram-library-link = { git = "ssh://github.com/ConnorGray/wl-library-link.git" }

# Support libraries
wl-pattern-match = { git = "ssh://git@stash.wolfram.com:7999/~connorg/wl-pattern-match.git" }
```

See the [Cargo manifest documentation][cargo-manifest-docs] for a complete description of
the Cargo TOML file.

### Using `#[wolfram_library_function]` to generate a wrapper automatically

Next, import and use the `#[wolfram_library_function]` macro into your Rust code.

```rust
// ### lib.rs

use wl_expr::{Expr, Number, FromExpr, forms::{FromExpr, ToExpr, List, from_expr::FormError}};
use wolfram_library_link as wll;

#[derive(FromExpr)]
#[pattern(numbers:{___})]
struct Numbers {
    numbers: List<Number>,
}

#[wolfram_library_link::wolfram_library_function]
pub fn sum_of_numbers(arguments: Vec<Expr>) -> Expr {
    let Numbers { numbers } = match Numbers::from_expr(&arguments[0]) {
        Ok(numbers) => numbers,
        Err(err) => return Expr! {
            Failure["ArgumentShape", <|
                "Message" -> %[format!("{}", FormError::from(err))]
            |>]
        },
    };

    let List(numbers) = numbers;

    let mut sum: f64 = 0.0;

    for number in numbers {
        wll::evaluate(&Expr! { Echo['number] });

        match number {
            Number::Integer(int) => sum += int as f64,
            Number::Real(real) => sum += *real,
        }
    }

    Expr::number(Number::Real(wl_expr::F64::new(sum).expect("got NaN")))
}
```

### Creating a library which is usable from Rust and Wolfram

`crate-type = ["rlib", "cdyib"]`

The [cargo-paclet][cargo-paclet] command-line utility can be used to automate the process
of building a Paclet from a Rust library.

# Examples

The [./wolfram-library-link/examples](./wolfram-library-link/examples) subdirectory contains sample
programs demonstrating features of the `wolfram-library-link` API.

* [raw_librarylink_function.rs](wolfram-library-link/examples/raw_librarylink_function.rs)
  - Demonstrates how to write "raw" LibraryLink functions, using the `extern "C"` ABI
    and the raw `MArgument` type.
* [raw_wstp_function.rs](wolfram-library-link/examples/raw_wstp_function.rs)
  - Demonstrates how to write "raw" LibraryLink WSTP functions, using the `extern "C"` ABI,
    raw `WSLINK` type, and low-level WSTP operations.

[wstp]: https://stash.wolfram.com/users/connorg/repos/wstp/browse
[cargo-paclet]: https://stash.wolfram.com/users/connorg/repos/cargo-paclet/browse
[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html
[library-function-load]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
[cargo-manifest-docs]: https://doc.rust-lang.org/cargo/reference/manifest.html