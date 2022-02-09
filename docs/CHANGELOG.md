# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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


<!-- This needs to be updated for each tagged release. -->
[Unreleased]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.2...HEAD

[0.1.2]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/WolframResearch/wolfram-library-link-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/WolframResearch/wolfram-library-link-rs/releases/tag/v0.1.0
