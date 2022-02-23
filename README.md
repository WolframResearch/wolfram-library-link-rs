# wolfram-library-link

[![Crates.io](https://img.shields.io/crates/v/wolfram-library-link.svg)](https://crates.io/crates/wolfram-library-link)
![License](https://img.shields.io/crates/l/wolfram-library-link.svg)
[![Documentation](https://docs.rs/wolfram-library-link/badge.svg)](https://docs.rs/wolfram-library-link)

<h4>
  <a href="https://docs.rs/wolfram-library-link">API Documentation</a>
  <span> | </span>
  <a href="https://github.com/WolframResearch/wolfram-library-link-rs/blob/master/docs/CHANGELOG.md">Changelog</a>
  <span> | </span>
  <a href="https://github.com/WolframResearch/wolfram-library-link-rs/blob/master/docs/CONTRIBUTING.md">Contributing</a>
</h4>

Bindings to the Wolfram LibraryLink interface, making it possible to call Rust code
from the Wolfram Language.

This library is used for writing Rust programs that can be loaded by the Wolfram
LibraryLink family of functions, specifically by
[`LibraryFunctionLoad[]`][ref/LibraryFunctionLoad].

#### Features

  * Efficiently call Rust functions from Wolfram code.
  * Pass arbitrary Wolfram expressions to and from Rust code.
  * Evaluate Wolfram expressions from Rust code.
  * Respond to Wolfram [abort][interrupts] requests while in Rust code.
  * Safe API for the Wolfram Symbolic Transfer Protocol, using the [`wstp`][wstp] crate.

Follow the [**Quick Start**](./docs/QuickStart.md) guide to begin using `wolfram-library-link`.

See [**Why Rust?**](./docs/WhyRust.md) for an overview of some of the advantages Rust has
when writing native code for use from the Wolfram Language: performance, memory and thread
safety, high-level features, and more.

[interrupts]: https://reference.wolfram.com/language/tutorial/InterruptingCalculations.html

## Quick Examples

The examples in this section are written with two back-to-back code blocks. The first
shows the Rust code, and the second shows the Wolfram Language code needed to load and use
the related Rust function(s).

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

## Example Programs

The [wolfram-library-link/examples](./wolfram-library-link/examples) subdirectory
contains sample programs demonstrating features of the `wolfram-library-link` API.

Rust code                                                                          | Wolfram Language code                                                   | Demonstrates ...
-----------------------------------------------------------------------------------|-------------------------------------------------------------------------|-------------------------------
[basic_types.rs](wolfram-library-link/examples/basic_types.rs)                     | [BasicTypes.wlt](RustLink/Examples/BasicTypes.wlt)                      | how to write Rust *LibraryLink* functions utilizing the basic, native types that can be passed efficiently, like integers, floating-point real numbers, and strings.
[numeric_arrays.rs](wolfram-library-link/examples/numeric_arrays.rs)               | [NumericArrays.wlt](RustLink/Examples/NumericArrays.wlt)                | how the `NumericArray` data type can be used to efficiently pass large multi-dimensional arrays of uniform numeric data.
[wstp.rs](wolfram-library-link/examples/wstp.rs)                                   | [WSTP.wlt](RustLink/Examples/WSTP.wlt)                                  | how WSTP [[`Link`]]s can be used to pass arbitrary expressions to and from LibraryLink functions.
[aborts.rs](wolfram-library-link/examples/aborts.rs)                               | [Aborts.wlt](RustLink/Examples/Aborts.wlt)                              | how Rust code can respond to Wolfram [abort requests][interrupts].
[async_file_watcher.rs](wolfram-library-link/examples/async/async_file_watcher.rs) | [AsyncExamples.wlt](RustLink/Examples/AsyncExamples.wlt)                | how Rust code can generate asynchronous events that trigger Wolfram evaluations to process the event.
[managed.rs](wolfram-library-link/examples/exprs/managed.rs)                       | [ManagedExpressions.wlt](RustLink/Examples/ManagedExpressions.wlt)      | how the managed expression API can be used to free library data when a Wolfram expression is deallocated.
[data_store.rs](wolfram-library-link/examples/data_store.rs)                       | [DataStore.wlt](RustLink/Examples/DataStore.wlt)                        | how the `DataStore` data type can be used to efficiently pass arbitrary expression-like heterogenous structures made up of native *LibraryLink* data types.

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

[wstp]: https://crates.io/crates/wstp
[wolfram-expr]: https://crates.io/crates/wolfram-expr
[wolfram-app-discovery]: https://crates.io/crates/wolfram-app-discovery
[library-link]: https://reference.wolfram.com/language/guide/LibraryLink.html

[wad-configuration]: https://github.com/WolframResearch/wolfram-app-discovery-rs#configuration

[ref/LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html
[ref/LibraryDataType]: https://reference.wolfram.com/language/ref/LibraryDataType.html
[ref/NumericArray]: https://reference.wolfram.com/language/ref/NumericArray.html

## Building `wolfram-library-link`

`wolfram-library-link` depends on the [`wstp`][wstp] crate for bindings to the Wolfram
Symbolic Transfer Protocol (WSTP). Building the `wstp` crate requires access to the
WSTP SDK, which provides the WSTP static library. `wstp` uses [`wolfram-app-discovery`][wolfram-app-discovery] to
locate a local installation of the Wolfram Language that contains a suitable copy of the
WSTP SDK. If the WSTP SDK cannot be located, `wstp` will fail to build, and consequently,
so will `wolfram-library-link`.

If you have installed the Wolfram Language to a location unknown to `wolfram-app-discovery`,
you may specify the installed location manually by setting the `WOLFRAM_APP_DIRECTORY`
environment variable. See [Configuring wolfram-app-discovery][wad-configuration] for details.

## Related Links

#### Related crates

* [`wstp`][wstp] — bindings to the Wolfram Symbolic Transfer Protocol, used for passing
  arbitrary Wolfram expressions between programs.
* [`wolfram-expr`][wolfram-expr] — native Rust representation of Wolfram Language
  expressions.
* [`wolfram-app-discovery`][wolfram-app-discovery] — utility for locating local
  installations of Wolfram applications and the Wolfram Language.

#### Related documentation

* [*Wolfram LibraryLink User Guide*](https://reference.wolfram.com/language/LibraryLink/tutorial/Overview.html)
* [*Introducing C++ and the Wolfram Language with LibraryLinkUtilities*](https://community.wolfram.com/groups/-/m/t/2133603), a C++ wrapper around the *LibraryLink* API.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note: Licensing of the WSTP library linked by the [wstp][wstp] crate is covered by the
terms of the
[MathLink License Agreement](https://www.wolfram.com/legal/agreements/mathlink.html).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](./docs/CONTRIBUTING.md) for more information.

### Developer Notes

See [**Development.md**](./docs/Development.md) for instructions on how to perform common
development tasks when contributing to the `wolfram-library-link` crate.

See [**Maintenance.md**](./docs/Maintenance.md) for instructions on how to keep
`wolfram-library-link` up to date as new versions of the Wolfram Language are released.
