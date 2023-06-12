# Maintenance

This document describes tasks necessary to maintain the `wolfram-library-link` and
`wolfram-library-link-sys` crates over time. This document is informational and intended
for the maintainer of these crates; users of these crates do not need to read this
document.

## Release

### Check that common feature combinations compile successfully

```shell
$ cargo check --all-features --all-targets --tests --examples --benches
```

```shell
$ cargo check --no-default-features --all-targets --tests --examples --benches
```


## Generating `wolfram-library-link-sys` bindings

After every Wolfram Language release, the pre-generated bindings stored in the
[`wolfram-library-link-sys/generated/`](../wolfram-library-link-sys/generated/) directory
need to be updated. That directory has the following layout:

```text
generated/<$VersionNumber.$ReleaseNumber>/<$SystemID>/LibraryLink_bindings.rs
```

To quickly generate bindings for the current platform, execute:

```shell
$ cargo +nightly xtask gen-bindings
```

The new bindings will automatically be placed in the appropriate
sub-directory, and should be commited to this repository. The script will need to be run
on each supported platform.

#### Example

From the `RustLink` repository root directory:

```
$ export WOLFRAM_APP_DIRECTORY=/Applications/Wolfram/12.2.x/Mathematica-12.2.0.app
$ cargo +nightly xtask gen-bindings
```

will re-generate the `wolfram-library-link-sys/generated/12.2.0/MacOSX-x86-64/LibraryLink_bindings.rs`
file.

## Updating build.rs bindings to use on docs.rs

When `wolfram-library-link-sys` is built in the <docs.rs> environment, some special logic
is required to work around the fact that no Wolfram applications are available to query
for the Wolfram version number.

At the moment, the [`wolfram-library-link-sys/build.rs`](../wolfram-library-link-sys/build.rs)
file hard-codes a Wolfram version number and System ID to use as the bindings to display
on docs.rs. That version number should be updated each time new `wolfram-library-link-sys`
bindings are generated.