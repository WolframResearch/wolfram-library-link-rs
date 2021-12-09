# wolfram-library-link

This library offers bindings to Rust code from the Wolfram language.

This library is used for writing Rust programs that can be loaded by the Wolfram language
LibraryLink family of functions, specifically by
[`LibraryFunctionLoad[]`][ref/LibraryFunctionLoad].

Features:

  * Call Rust functions from the Wolfram language.
  * Pass general Wolfram language expressions to and from Rust code.
  * Evaluate Wolfram expressions from Rust code.
  * Check for and respond to Wolfram language aborts while in Rust code.
  * Seamlessly construct native Rust datastructures from expressions using the pattern
    language via `derive(FromExpr)`.
  * Safe API for the Wolfram Symbolic Transport Protocol, using the [`wstp`][wstp] crate.

Advantages over the LibraryLink/WSTP C API:

  * The API is easier to use
    - No need to worry about managing allocations
    - No potential memory unsafety
    - Easy and safe method call to evaluate an expression from Rust code
  * Ability to use a safer, but still just as performant, modern programming language

Follow the [**Quick Start**](./docs/QuickStart.md) guide to begin using `wolfram-library-link`.

# Quick Examples

Each of the examples in this section are written with two back-to-back code blocks. The
first shows the Rust code, the second shows the Wolfram Language code needed to load and
use the related Rust function(s).

#### Basic data types

```rust
use wolfram_library_link::export;

export![square(_)];

fn square(x: i64) -> i64 {
	x * x
}
```

```wolfram
square = LibraryFunctionLoad["...", "square", {Integer}, Integer];

square[5]
```

See also: [`LibraryFunctionLoad`][ref/LibraryFunctionLoad]

#### Efficient numeric arrays

Create an array of a million integers in Wolfram Language and compute the total using
Rust:

```rust
use wolfram_library_link::{export, NumericArray};

export![total(_)];

fn total(array: &NumericArray<i64>) -> i64 {
	array.as_slice().into_iter().sum()
}
```

```wolfram
total = LibraryFunctionLoad[
	"...",
	"square",
	{LibraryDataType[NumericArray, "Integer64"]},
	Integer
];

total[NumericArray[Range[1000000], "Integer64"]]
```

See also: [`NumericArray`][ref/NumericArray], [`LibraryDataType`][ref/LibraryDataType]

# Example Programs

The [./wolfram-library-link/examples](./wolfram-library-link/examples) subdirectory
contains sample programs demonstrating features of the `wolfram-library-link` API.

* [raw_librarylink_function.rs](wolfram-library-link/examples/raw_librarylink_function.rs)
  - Demonstrates how to write "raw" LibraryLink functions, using the `extern "C"` ABI
    and the raw `MArgument` type.
* [raw_wstp_function.rs](wolfram-library-link/examples/raw_wstp_function.rs)
  - Demonstrates how to write "raw" LibraryLink WSTP functions, using the `extern "C"` ABI,
    raw `WSLINK` type, and low-level WSTP operations.

In addition to the polished high-level examples, the
[wolfram-library-link/examples/tests/](wolfram-library-link/examples/tests/) directory
contains test code for a more exhaustive range of functionality and behavior, and may be a
useful additional reference. The [RustLink/Tests/](./RustLink/Tests/) directory contains
the Wolfram Language unit testing logic that loads and calls the test functions.

[wstp]: https://stash.wolfram.com/users/connorg/repos/wstp/browse
[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html

[ref/LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
[ref/LibraryDataType]: https://reference.wolfram.com/language/ref/LibraryDataType.html
[ref/NumericArray]: https://reference.wolfram.com/language/ref/NumericArray.html

# Related Links

* [*Wolfram LibraryLink User Guide*](https://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html)
* [*Introducing C++ and the Wolfram Language with LibraryLinkUtilities*](https://community.wolfram.com/groups/-/m/t/2133603), a C++ wrapper around the *LibraryLink* API.
