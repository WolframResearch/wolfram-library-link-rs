# Functionality Overview

## Function Types

#### Native functions

#### WSTP functions

#### TODO: Expr functions

## WSTP Functions

### Panic `Failure[..]`s

When a WSTP function [panics][panics], a [`Failure["RustPanic", ...]`][ref/Failure] object
will be returned to the Wolfram Language.

[panics]: https://doc.rust-lang.org/std/macro.panic.html
[ref/Failure]: https://reference.wolfram.com/language/ref/Failure.html

##### Example

```rust
# mod scope {
use wolfram_library_link::{self as wll, wstp::Link};

wll::export_wstp![sqrt];

fn sqrt(link: &mut Link) {
    let arg_count = link.test_head("List").unwrap();

    if arg_count != 1 {
        panic!("expected 1 argument, got {}", arg_count);
    }

    let arg = link.get_f64().expect("expected Real argument");

    if arg.is_negative() {
        panic!("cannot get the square root of a negative number");
    }

    let value = arg.sqrt();

    link.put_f64(value).unwrap()
}
# }
```

```wolfram
sqrt = LibraryFunctionLoad["...", "sqrt", LinkObject, LinkObject];

sqrt[]

(* Returns:
    Failure["RustPanic", <|
        "MessageTemplate" -> "Rust LibraryLink function panic: `message`",
        "MessageParameters" -> <| "message" -> "expected 1 argument, got 0" |>,
        "SourceLocation" -> "<...>",
        "Backtrace" -> Missing["NotEnabled"]
    >]
*)
```

#### Backtraces

`wolfram-library-link` can optionally capture a stack backtrace for panics that occur
within Rust LibraryLink functions. This behavior is disabled by default, but can be
enabled by setting the `LIBRARY_LINK_RUST_BACKTRACE` environment variable to `"True"`:

```wolfram
SetEnvironment["LIBRARY_LINK_RUST_BACKTRACE" -> "True"]
```

##### Note on panic hooks

`wolfram-library-link` captures information about panics by setting a custom Rust
[panic hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html).