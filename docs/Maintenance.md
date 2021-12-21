# Maintenance

This document describes tasks necessary to maintain the `wolfram-library-link` and
`wolfram-library-link-sys` over time.

## Generating `wolfram-library-link-sys` bindings

After every Wolfram Language release, the pre-generated bindings stored in the
[`wolfram-library-link-sys/generated/`](../wolfram-library-link-sys/generated/) directory
need to be updated. That directory has the following layout:

```text
generated/<$VersionNumber.$ReleaseNumber>/<$SystemID>/LibraryLink_bindings.rs
```

The [`scripts/generate_versioned_bindings.rs`](../scripts/generate_versioned_bindings.rs)
script (invoked using `$cargo make gen-bindings`) can be used to quickly generate bindings
for the current platform. The new bindings will automatically be placed in the appropriate
sub-directory, and should be commited to this repository. The script will need to be run
on each supported platform.

TODO: Mention usage of `RUST_WOLFRAM_LOCATION` env variable used by wolfram-app-discovery
to configure the Wolfram Engine (Wolfram Engine/Wolfram Desktop/Mathematica products)
location to use.