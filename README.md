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
  * TODO: WSTP bindings

## Usage

First, ensure that your project's `Cargo.toml` is correctly configured. This means:

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
```

See the [Cargo manifest documentation][cargo-manifest-docs] for a complete description of
the Cargo TOML file.

### Using the `generate_wrapper!()` macro

Next, import and use the `generate_wrapper!()` macro in your Rust code.

```rust
// ### main.rs

use wl_expr::Expr;
use wl_library_link::generate_wrapper;

generate_wrapper![GET_HEAD # get_normal_head(e: Expr) -> Expr];

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

### Writing a LibraryLink ABI compatible function manually

This example makes use of the [wl-wstp][wl-wstp] library to provide a safe wrapper around
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
            let err = wlexpr! { Failure["WSTP Error", <| "Message" -> 'err |>] };
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
function = LibraryFunctionLoad["/path/to/library", "wstp_function", LinkObject, LinkObject]
```

Finally, build the library by executing the following commands in the terminal:

```shell
$ cargo build
```

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
[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html
[library-function-load]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
[cargo-manifest-docs]: https://doc.rust-lang.org/cargo/reference/manifest.html