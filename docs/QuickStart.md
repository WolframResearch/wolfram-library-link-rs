# Quick Start to Wolfram *LibraryLink* for Rust

Wolfram *LibraryLink* is an interface enabling efficient communication between the
Wolfram Language and loadable dynamic libraries that can provide new high-performance
algorithms, connections to external tools and services, links to other languages, and
more.

This Quick Start will walk you through how to create a new Rust library that can
be called from the Wolfram Language. This guide will cover:

  * creating a new Rust library crate
    - configuring the crate to generate a dynamic library when built
    - adding the appropriate `wolfram-library-link` dependency
  * writing basic Rust functions that can be called via the *LibraryLink* interface.
  * building the library and loading it via [`LibraryFunctionLoad`][ref/LibraryFunctionLoad]

The library we write will be compiled into a dynamic library, whose functions can be
called directly from the Wolfram Language.

Instructions for installing and setting up Rust can be found at:

&nbsp;&nbsp;&nbsp;&nbsp;<https://www.rust-lang.org/tools/install>

[ref/LibraryFunctionLoad]: https://reference.wolfram.com/language/ref/LibraryFunctionLoad.html

## Create a new library crate

Assuming that have already installed Rust, you can create a new Rust library from the
command line by running:

```shell
$ cargo new --lib my-package
```

This will automatically create a new directory called `my-package`. The `my-package`
directory will contain a `Cargo.toml` file, which contains the specification describing
the new crate.

> Feel free to choose a name other than `my-package`, though be sure to update the name
> appropriately while following the instructions in this guide.

#### Configure the crate

By default, Rust libraries are only built into a format usable as a dependency from other
Rust crates. If we want to build a standalone dynamic library, we'll need to specify that
in the `Cargo.toml` file by setting the
[`crate-type`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field)
property.

Add the following lines to the `Cargo.toml` file, first creating a new `[lib]` section
following the existing `[package]` section:

```toml
[lib]
crate-type = ["cdylib"]
```

Setting `crate-type` to `"cdylib"` instructs the `cargo` build system to produce a dynamic
library that is suitable for loading from C/C++ programs.

> See the [Linkage](https://doc.rust-lang.org/reference/linkage.html) section of The Rust
> Reference for more information on the different crate type options.

Next, declare that this crate depends on `wolfram-library-link` by adding the
following line to the `[dependencies]` section:

```toml
wolfram-library-link = "X.X.X"
```

A correctly configured Cargo.toml looks like:

```toml
### Cargo.toml

[package]
# This can be whatever you like, as long as it's a valid crate identifier.
name = "my-package"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wolfram-library-link = "X.X.X"
```

> See the [Cargo manifest documentation][cargo-manifest-docs] for a complete description
> of the Cargo.toml file.

## Write a basic Rust function

The `cargo new` command used to create the crate will have automatically created a
`my-package/src/lib.rs` file. Replace the existing sample content in that file with
the following Rust code:

```rust
use wolfram_library_link::export;

export![square(_)];

fn square(x: i64) -> i64 {
    x * x
}
```

This is all that is needed to expose a basic Rust function to the Wolfram Language via
the *LibraryLink* interface. The `export![]` macro automatically generates an efficient
wrapper function that uses the low-level interface expected by *LibraryLink*.

## Building and using the library

Now that we've written a basic library, we can compile it from the command line by
running:

```shell
$ cargo build
```

This will automatically fetch any dependencies, and build the dynamic library we specified.
The resulting (unoptimized) library will be located at:

```text
my-package/target/debug/libmy_package.dylib
```

> The name of the `my_package` dynamic library file will vary depending on your operating
> system:
>
> * macOS: `libmy_package.dylib`
> * Windows: `my_package.dll`
> * Linux: `libmy_package.so`

This library can be loaded directly into the Wolfram Language by evaluating:

```wolfram
square = LibraryFunctionLoad[
	"/path/to/libmy_package.dylib",
	"square",
	{Integer},
	Integer
];

square[5]
```

[cargo-manifest-docs]: https://doc.rust-lang.org/cargo/reference/manifest.html
[cargo-paclet]: https://stash.wolfram.com/users/connorg/repos/cargo-paclet/browse