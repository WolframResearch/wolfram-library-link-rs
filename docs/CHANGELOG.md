# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]



## [0.2.2] – 2022-03-28

### Added

* Added a new [`wolfram_library_link::docs`](https://docs.rs/wolfram-library-link/0.2.2/wolfram_library_link/docs/index.html)
  module, and an initial 'How To' documentation page describing how to convert between
  Rust types and Wolfram expressions. ([#31])

  * [How To: Convert Between Rust and Wolfram Types](https://docs.rs/wolfram-library-link/0.2.2/wolfram_library_link/docs/converting_between_rust_and_wolfram_types/index.html)



## [0.2.1] – 2022-03-09

### Added

* Added [`exported_library_functions_association()`](https://docs.rs/wolfram-library-link/0.2.1/wolfram_library_link/fn.exported_library_functions_association.html).
  ([#26])

  This function returns an [`Expr`](https://docs.rs/wolfram-expr/0.1.1/wolfram_expr/struct.Expr.html)
  containing an Association of the form `<| name_?StringQ -> func_ |>`, with an entry for
  each library function exported using `#[export(..)]`.

  Arguments that are applied to `func` will be used to call the compiled library function.

* Added `#[export(hidden)]` annotation.

  Exported functions with the `hidden` annotation will not be included in the
  Association returned by `exported_library_functions_association()`.

`exported_library_functions_association()` and `#[export(hidden)]` are alternatives to
the `generate_loader![]` macro. The `generate_loader![]` macro is convenient, but I think
it hides too many details about how it works. It's too much magic.

Together, these two new features can be used by the library author to define a loader
function for their own library, which would typically look like:

```rust
use wolfram_library_link::{self as wll, export, expr::Expr};

#[export(wstp, hidden)]
fn load_my_lib_funcs(_args: Vec<Expr>) -> Expr {
    return wll::exported_library_functions_association(None);
}

#[export]
fn square(x: i64) -> i64 {
    x * x
}
```

and which could be used from the Wolfram Language by evaluating:

```wolfram
loadLibraryFunctions = LibraryFunctionLoad[
    "<library path>",
    "load_my_lib_funcs",
    LinkObject,
    LinkObject
];

$functions = loadLibraryFunctions[];
```

Then, any function exported from the library could be called by accessing the named values
in `$functions`:

```wolfram
(* Call the `square()` function exported by this library. *)
$functions["square"][5]
```



## [0.2.0] – 2022-03-07

### Added

* Added new `#[export(..)]` attribute macro.  ([#23])

  Export a native function:

  ```rust
  use wolfram_library_link::export;

  #[export]
  fn square(x: i64) -> i64 {
      x * x
  }
  ```

  Export a WSTP function:

  ```rust
  use wolfram_library_link::{export, wstp::Link};

  #[export(wstp)]
  fn total(link: &mut Link) {
      let arg_count = link.test_head("List").unwrap();

      let mut total = 0;

      for _ in 0..arg_count {
          total += link.get_i64().unwrap();
      }

      link.put_i64(total).unwrap();
  }
  ```

### Changed

* Changed `wolfram-library-link-sys` to generate the Rust bindings to `WolframLibrary.h`
  at compile time.  ([#24])

  This ensures that the `wolfram-library-link` and `wolfram-library-link-sys` crates can
  compile against the widest possible range of suppported Wolfram Language versions.

### Removed

* Removed the `export![]` and `export_wstp![]` declarative macros. These have been
  replaced by `#[export(..)]`.  ([#23])



## [0.1.2] – 2022-02-08

### Fixed

* Fix `wolfram-library-link-sys/build.rs` failure when building in the <docs.rs> build
  environment, where no Wolfram applications are available to query.  ([#17])



## [0.1.1] – 2022-02-08

### Fixed

* Update `wstp` dependency to fix <docs.rs> build failures caused by earlier versions of
  `wstp-sys`.  ([#16])
* Fix missing `"full"` feature needed by the `syn` dependency of
  `wolfram-library-link-macros`.  ([#16])



## [0.1.0] – 2022-02-08

Initial release. `wolfram-library-link-sys` was the only crate published in this release,
due to a [docs.rs build failure](https://docs.rs/crate/wolfram-library-link-sys/0.1.0)
caused by bugs present in early versions of `wolfram-app-discovery` and `wstp-sys`.




[#16]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/16
[#17]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/17

<!-- v0.2.0 -->
[#23]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/23
[#24]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/24

<!-- v0.2.1 -->
[#26]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/26

<!-- v0.2.2 -->
[#31]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/31


<!-- This needs to be updated for each tagged release. -->
[Unreleased]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.2...HEAD

[0.2.2]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WolframResearch/wolfram-library-link-rs/releases/tag/v0.1.0
