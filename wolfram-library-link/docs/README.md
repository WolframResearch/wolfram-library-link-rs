The files in this directory are included directly in the `wolfram-library-link` crate
documentation using:

```rust
#[doc = include_str!("<..>/docs/included/<file>.md")]
```

When viewed as markdown files in an online repository file browser, intra-doc links will
not work correctly.