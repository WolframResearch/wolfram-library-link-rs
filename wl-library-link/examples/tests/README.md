# wl-library-link/examples/tests/

This directory contains tests for wl-library-link functionality, which are declared
as cargo examples so that they produce dynamic libraries which can be loaded by
standard Wolfram Language MUnit tests.

This is necessary because much of wl-library-link depends on being dynamically linked
into a running Wolfram Language Kernel, so tests for that functionality must necessarily
be initiated by the Kernel.
