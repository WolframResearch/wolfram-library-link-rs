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
  * Seamlessly construct native Rust datastructures from expressions using the pattern
    language via `derive(FromExpr)`.
  * Safe API for the WSTP, using [`wl-wstp`][wl-wstp]

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
  * Adding `wl-expr` and `wl-library-link` as dependencies

By setting `crate-type` to `cdylib` we tell `cargo` to build a dynamic library, which
will be loadable using Wolfram [LibraryLink][library-link].

The `wl-expr` and `wl-library-link` dependencies provide, respectively, an `Expr` type,
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
wl-library-link = { git = "ssh://github.com/ConnorGray/wl-library-link.git" }

# Support libraries
wl-lang = { git = "ssh://git@stash.wolfram.com:7999/~connorg/wl-lang.git" }
wl-expr-macro = { git = "ssh://git@stash.wolfram.com:7999/~connorg/wl-expr-macro.git" }
wl-pattern-match = { git = "ssh://git@stash.wolfram.com:7999/~connorg/wl-pattern-match.git" }
```

See the [Cargo manifest documentation][cargo-manifest-docs] for a complete description of
the Cargo TOML file.

### Using `#[wolfram_library_function]` to generate a wrapper automatically

Next, import and use the `#[wolfram_library_function]` macro into your Rust code.

```rust
// ### lib.rs

use wl_expr::{Expr, Number};
use wl_expr_macro::Expr;
use wl_library_link::WolframEngine;
use wl_lang::{FromExpr, forms::{FromExpr, ToExpr, List, from_expr::FormError}};

#[derive(FromExpr)]
#[pattern(Pattern[numbers, {___}])]
struct Numbers {
    numbers: List<Number>,
}

#[wl_library_link::wolfram_library_function]
pub fn sum_of_numbers(engine: &WolframEngine, arguments: Vec<Expr>) -> Expr {
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
        engine.evaluate(&Expr! { Echo['number] });

        match number {
            Number::Integer(int) => sum += int as f64,
            Number::Real(real) => sum += *real,
        }
    }

    Expr::number(Number::Real(wl_expr::F64::new(sum).expect("got NaN")))
}
```

### Writing a LibraryLink ABI compatible function manually

This example makes use of the [`wl-wstp`][wl-wstp] library to provide a safe wrapper around
around the WSTP link object, which can be used to read the argument expression and write
out the return expression.

```rust
use wl_library_link::{WolframLibraryData, LIBRARY_NO_ERROR, LIBRARY_FUNCTION_ERROR};
use wl_wstp::WSTPLink;
use wl_wstp_sys::WSLINK;

#[no_mangle]
pub extern "C" fn wstp_function(
    _lib: WolframLibraryData,
    unsafe_link: WSLINK,
) -> c_uint {
    let link = unsafe {
        WSTPLink::new(unsafe_link)
    };

    let expr = match link.get_expr() {
        Ok(expr) => expr,
        Err(err) => {
            let err = Expr! { Failure["WSTP Error", <| "Message" -> 'err |>] };
            match link.put_expr(&err) {
                Ok(()) => return LIBRARY_NO_ERROR,
                Err(_) => return LIBRARY_FUNCTION_ERROR,
            }
        },
    };

    let expr_string = format!("Input: {}", expr.to_string());

    match link.put_expr(&Expr::string(expr_string)) {
        Ok(()) => LIBRARY_NO_ERROR,
        Err(_) => LIBRARY_FUNCTION_ERROR,
    }
}
```

Then, in Wolfram:

```wolfram
function = LibraryFunctionLoad["/path/to/library.dylib", "wstp_function", LinkObject, LinkObject]
```

Finally, build the library by executing the following commands in the terminal:

```shell
$ cargo build
```

The [cargo-paclet][cargo-paclet] command-line utility can be used to automate the process
of building a Paclet from a Rust library.

### Creating a library which is usable from Rust and Wolfram

`crate-type = ["rlib", "cdyib"]`

## Using the raw LibraryLink and WSTP APIs

```rust
use std::os::raw::{c_int, c_uint};

use wl_library_link::{
    mint, MArgument, WolframLibraryData, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR,
};
use wl_wstp_sys::{
    WSGetInteger, WSNewPacket, WSPutInteger, WSTestHead, WSLINK,
};

#[no_mangle]
pub unsafe extern "C" fn demo_function(
    _lib_data: WolframLibraryData,
    _arg_count: mint,
    _args: MArgument,
    res: MArgument,
) -> c_uint {
    *res.real = 42.42;
    0
}

#[no_mangle]
pub unsafe extern "C" fn demo_wstp_function(
    _lib: WolframLibraryData,
    link: WSLINK,
) -> c_uint {
    let mut i1: c_int = 0;
    let mut i2: c_int = 0;
    let mut len: c_int = 0;

    if WSTestHead(link, b"List\0".as_ptr() as *const i8, &mut len) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if len != 2 {
        return LIBRARY_FUNCTION_ERROR;
    }

    if WSGetInteger(link, &mut i1) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if WSGetInteger(link, &mut i2) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }
    if WSNewPacket(link) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    let sum = i1 + i2;

    if WSPutInteger(link, sum) == 0 {
        return LIBRARY_FUNCTION_ERROR;
    }

    return LIBRARY_NO_ERROR;
}
```

[wl-wstp]: https://stash.wolfram.com/users/connorg/repos/wl-wstp/browse
[cargo-paclet]: https://stash.wolfram.com/users/connorg/repos/cargo-paclet/browse
[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html
[library-function-load]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
[cargo-manifest-docs]: https://doc.rust-lang.org/cargo/reference/manifest.html