This folder contains a Dockerfile to build a reusable image that bundles
the WolframEngine runtime base image with the Rust toolchain and required
build dependencies. Use this when you want to regenerate LibraryLink Rust
bindings without re-downloading and re-installing toolchain each run.

Build the image (run once):

    docker build -f scripts/Dockerfile.gen-bindings -t wll-gen-bindings:latest .

Run against the repository (mounts the repo and runs the xtask generator):

    docker run --rm -u 0 -v $(pwd):/work -w /work wll-gen-bindings:latest \
      /bin/bash -lc ". /usr/local/cargo/env; rustup target add x86_64-unknown-linux-gnu || true; cargo +nightly xtask gen-bindings --target x86_64-unknown-linux-gnu"

Notes:
- The image is based on WolframEngine which includes the Mathematica headers.
- We run as root in the container to ensure apt and rustup can install.
- If you have a local Wolfram install and prefer to mount that instead of using
  the runtime from the base image, you can mount it at /usr/local/Wolfram.
