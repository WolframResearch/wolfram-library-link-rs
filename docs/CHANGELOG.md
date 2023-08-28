# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.2.10] – 2023-08-28

### Changed

* Always use pre-generated LibraryLink bindings. ([#57])

  Previously, wolfram-library-link-sys would use bindgen to generate bindings at
  compile time. Now, the bindings are pre-generated and built-in to
  wolfram-library-link-sys, without any loss in functionality.

  The build dependency on bindgen has been removed, simplifying the dependency
  tree and reducing compile times.

  When wolfram-library-link occasionally updates to support a new minimum
  LibraryLink version, the bindings will need to be regenerated as described in
  [docs/Maintenance.md](./Maintenance.md).

* Update wstp dependency to v0.2.8, which also replaces build-time use of
  bindgen with pre-generated bindings. ([#59])

* *Developer:* Replace custom RunTests.wls script with instructions on how to
  use the slightly more standardized `wolfram-cli paclet test` tool. ([#58])

### Fixed

* Fixed build failure caused by assumption that `c_char` is `i8` on every
  platform, which is not true.



## [0.2.9] – 2023-02-03

### Added

* Add logging support to `wolfram-library-link-sys/build.rs`. ([#54])

  [wolfram-app-discovery v0.4.3](https://github.com/WolframResearch/wolfram-app-discovery-rs/blob/master/docs/CHANGELOG.md#043--2023-02-03)
  added support for logging via the Rust [`log`](https://crates.io/crates/log)
  logging facade library. `wolfram-library-link-sys/build.rs` uses
  wolfram-app-discovery to find `WolframLibrary.h`.

  Logging messages from `wolfram-library-link-sys/build.rs` can now be enabled
  by setting the `RUST_LOG` environment to an appropriate value, as documented
  in the [`env_logger`](https://docs.rs/env_logger) crate documentation. This
  can help debug discovery errors that occur during a build.

### Changed

* Reduce minimal configurable set of dependencies by adding new cargo features.
  ([#53])

  This release adds two new cargo features:

  * `panic-failure-backtraces`
  * `automate-function-loading-boilerplate`

  which are enabled by default.

  These features are used to control whether the following (now optional)
  dependencies are enabled:

  * `backtrace`
  * `inventory`
  * `process_path`

  Making these dependnecies optional reduces the minimal possible set of
  overall dependencies in projects that use wolfram-library-link.



## [0.2.8] – 2023-02-01

### Changed

* Update `wstp` and `wolfram-app-discovery` dependencies so that
  `wolfram-app-discovery` v0.4.1 is being used everywhere, which has improved
  Linux support, and fixes several bugs. ([#51])



## [0.2.7] – 2022-09-19

### Changed

* Update `wolfram-app-discovery` dependency from v0.2.1 to v0.3.0, to take
  advantage of the improved flexibility of the new API functions tailored for
  use in build scripts. ([#49])



## [0.2.6] – 2022-08-28

### Fixed

* Fixed `Failure["RustPanic", ..]` messages not being returned from
  `#[export(wstp)]` functions when a panic occurs after partial results had
  begun being written to the WSTP link. ([#46])

### Changed

* Clarified documentation problem
  [described in comment on #44](https://github.com/WolframResearch/wolfram-library-link-rs/issues/44#issuecomment-1153244113),
  the documentation for `exported_library_functions_association()` was unclear about
  when and how to use the `library` parameter. ([#47])



## [0.2.5] – 2022-06-11

### Fixed

* Fixed [issue #29](https://github.com/WolframResearch/wolfram-library-link-rs/issues/29), a compilation failure on Windows. ([#41], [#42])



## [0.2.4] – 2022-05-13

### Added

* Added a new 'How To' page describing how to perform Wolfram evaluations from Rust
  library functions. ([#38], [#39])

  * [How To: Evaluate Wolfram code from Rust](https://docs.rs/wolfram-library-link/0.2.4/wolfram_library_link/docs/evaluate_wolfram_code_from_rust/index.html)



## [0.2.3] – 2022-03-29

### Fixed

* Fixed docs.rs build failure in v0.2.2, caused by a `#[doc = include_str!(..)]` that
  fails on case-sensitive targets like the docs.rs Linux build host. ([#34])



## [0.2.2] – 2022-03-28

### Added

* Added a new [`wolfram_library_link::docs`](https://docs.rs/wolfram-library-link/0.2.3/wolfram_library_link/docs/index.html)
  module, and an initial 'How To' documentation page describing how to convert between
  Rust types and Wolfram expressions. ([#31])

  * [How To: Convert Between Rust and Wolfram Types](https://docs.rs/wolfram-library-link/0.2.3/wolfram_library_link/docs/converting_between_rust_and_wolfram_types/index.html)



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

<!-- v0.2.3 -->
[#34]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/34

<!-- v0.2.4 -->
[#38]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/38
[#39]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/39

<!-- v0.2.5 -->
[#41]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/41
[#42]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/42

<!-- v0.2.6 -->
[#46]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/46
[#47]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/47

<!-- v0.2.7 -->
[#49]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/49

<!-- v0.2.8 -->
[#51]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/51

<!-- v0.2.9 -->
[#53]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/53
[#54]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/54

<!-- v0.2.10 -->
[#57]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/57
[#58]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/58
[#59]: https://github.com/WolframResearch/wolfram-library-link-rs/pull/59


<!-- This needs to be updated for each tagged release. -->
[Unreleased]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.10...HEAD

[0.2.10]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.9...v0.2.10
[0.2.9]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WolframResearch/wolfram-library-link-rs/releases/tag/v0.1.0
