# wolfram-library-link/examples/tests/

This directory contains tests for wolfram-library-link functionality, which are declared
as cargo examples so that they produce dynamic libraries which can be loaded by
standard Wolfram Language MUnit tests.

This is necessary because much of wolfram-library-link depends on being dynamically linked
into a running Wolfram Language Kernel, so tests for that functionality must necessarily
be initiated by the Kernel. Writing these as cargo integration tests run using the
standard `cargo test` command would fail because their would be no Wolfram Kernel to load
and call the LibraryLink functions being tested.
