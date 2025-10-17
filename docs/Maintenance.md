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

### Generating bindings for multiple Linux platforms using Docker
A helper script is provided to run the binding generator inside Docker for a
single Linux target (x86_64) using the WolframResearch `wolframengine` image. This
is useful for CI or for maintainers who don't have the Wolfram runtime locally.
don't have every Linux architecture available locally.

Note: macOS bindings cannot be generated inside Docker because Mathematica/Wolfram
applications must be run on macOS to access the runtime headers and shared libraries.
You must still generate macOS bindings on macOS hosts and commit them.


From the repository root:

```shell
# Optionally point to a local Wolfram installation to mount inside the container
export WOLFRAM_APP_DIRECTORY="/path/to/Mathematica.app"
./scripts/gen_bindings_all.sh
```

The script will run the `wolframresearch/wolframengine:latest` container (or the
image specified by `WOLFRAM_DOCKER_IMAGE`), install Rust if necessary, and run
`cargo +nightly xtask gen-bindings` for the x86_64 Linux target. Generated files
will be placed under `wolfram-library-link-sys/generated/` and should be reviewed
and committed.


## Updating build.rs bindings to use on docs.rs

When `wolfram-library-link-sys` is built in the <docs.rs> environment, some special logic
is required to work around the fact that no Wolfram applications are available to query
for the Wolfram version number.

At the moment, the [`wolfram-library-link-sys/build.rs`](../wolfram-library-link-sys/build.rs)
file hard-codes a Wolfram version number and System ID to use as the bindings to display
on docs.rs. That version number should be updated each time new `wolfram-library-link-sys`
bindings are generated.