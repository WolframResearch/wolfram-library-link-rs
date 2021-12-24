# wolfram-library-link

Bindings to the Wolfram LibraryLink interface, making it possible to call Rust code
from the Wolfram Language.

This library is used for writing Rust programs that can be loaded by the Wolfram
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

Follow the [**Quick Start**](./docs/QuickStart.md) guide to begin using `wolfram-library-link`.

See [**Why Rust?**](./docs/WhyRust.md) for an overview of some of the advantages Rust has
when writing native code for use from the Wolfram Language: performance, memory and thread
safety, high-level features, and more.

# Quick Examples

The examples in this section are written with two back-to-back code blocks. The first
shows the Rust code, the second shows the Wolfram Language code needed to load and use the
related Rust function(s).

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

Rust code                                                                          | Wolfram Language code                                      | Demonstrates ...
-----------------------------------------------------------------------------------|------------------------------------------------------------|-------------------------------
[basic_types.rs](wolfram-library-link/examples/basic_types.rs)                     | [BasicTypes.wlt](RustLink/Examples/BasicTypes.wlt)         | how to write Rust *LibraryLink* functions utilizing the basic, native types that can be passed efficiently, like integers, floating-point real numbers, and strings.
[numeric_arrays.rs](wolfram-library-link/examples/numeric_arrays.rs)               | [NumericArrays.wlt](RustLink/Examples/NumericArrays.wlt)   | how the `NumericArray` data type can be used to efficiently pass large multi-dimensional arrays of uniform numeric data.
[wstp.rs](wolfram-library-link/examples/wstp.rs)                                   | [WSTP.wlt](RustLink/Examples/WSTP.wlt)                     | how WSTP [[`Link`]]s can be used to pass arbitrary expressions to and from LibraryLink functions.
[aborts.rs](wolfram-library-link/examples/aborts.rs)                               | [Aborts.wlt](RustLink/Examples/Aborts.wlt)                 | how Rust code can respond to Wolfram Language abort requests.
[async_file_watcher.rs](wolfram-library-link/examples/async/async_file_watcher.rs) | [AsyncExamples.wlt](RustLink/Examples/AsyncExamples.wlt)   | how Rust code can generate asynchronous events that trigger Wolfram Language evaluations to process the event.
[data_store.rs](wolfram-library-link/examples/data_store.rs)                       | [DataStore.wlt](RustLink/Examples/DataStore.wlt)           | how the `DataStore` data type can be used to efficiently pass arbitrary expression-like heterogenous structures made up of native *LibraryLink* data types.

#### Raw functions

These examples demonstrate how to write functions that use the "raw" low-level
*LibraryLink* and WSTP interfaces, using the `extern "C"` ABI, the low-level `MArgument`
and `WSLINK` types, and manual WSTP operations.

Rust code                                                            | Wolfram Language code
---------------------------------------------------------------------|-----------------
[raw_librarylink_function.rs](wolfram-library-link/examples/raw/raw_librarylink_function.rs) and [raw_wstp_function.rs](wolfram-library-link/examples/raw/raw_wstp_function.rs) | [RawFunctions.wlt](RustLink/Examples/RawFunctions.wlt)

#### Additional examples

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
