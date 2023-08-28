# Development

This document contains information useful to anyone wishing to modify the
`wolfram-library-link` crate.

## Run the tests

#### Build the `RustLink` crate

Unlike many Rust crates that use the [`cargo-test`][cargo-test] subcommand to execute
their tests, most `wolfram-library-link` tests are written in Wolfram, using the standard
`MUnit` testing framework. This is necessary because the test functions are designed to be
loaded via the Wolfram LibraryLink interface.

The testing code is split between two locations:

* The [wolfram-library-link/examples/tests/](../wolfram-library-link/examples/tests/) directory
  contains the Rust library test functions.
* The [RustLink/Tests/](../RustLink/Tests/) directory contains the Wolfram unit testing
  logic that loads and calls the Rust test functions.

The Rust tests are written as a standard
[cargo example](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples)
library, which is compiled into a dynamic library (`liblibrary_tests.dylib`) and loaded by
the Wolfram testing code. The test library is bundled into the built `RustLink` paclet,
along with the other example libraries.

> The [`cargo-make`](https://crates.io/crates/cargo-make) subcommand can be installed
> using:
>
> ```shell
> cargo install cargo-make
> ```

Build the `RustLink` paclet using:

```shell
cargo make paclet
```

Run the `wolfram-library-link` tests using:

```shell
wolfram-cli paclet test ./build/RustLink
```

This requires that the [unofficial wolfram-cli tool](https://github.com/ConnorGray/wolfram-cli)
is installed, and will run all of the Wolfram `.wlt` test files in the Tests directory,
and output the results to the terminal.

[cargo-test]: https://doc.rust-lang.org/cargo/commands/cargo-test.html