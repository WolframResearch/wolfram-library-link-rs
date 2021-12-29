# Why Rust?

#### *For Wolfram Language Developers*

The LibraryLink interface and WSTP library offer Wolfram Language developers the
capability to write performant low-level code, interface with external libraries, and
perform system operations, all easily callable from the Wolfram Language. This document
describes some reasons why the Rust programming language might be appealing to Wolfram
Language developers looking to write native code:

* **Performance** –
  Rust is fast. Powerful zero-cost abstractions enable Rust code to easily achieve
  performance on par with well-optimized C or C++ programs.

* **Safety** –
  Rust uses zero-cost abstractions to achieve memory-safety[^mem-safe] and
  thread-safety[^thread-safe] without garbage collection and unnecessary overhead. Rust
  empowers you to drop down into efficient native code without the risk of introducing
  crashes or undefined behavior into your Wolfram Language projects.

  Spend less time thinking about pointers and memory, and more time thinking about your
  problem domain.

* **Robust** –
  With rich algebraic data types, pattern matching, and simple error propagation idioms,
  Rust makes it easy to write code that carefully models complex data and handles error
  conditions, without excessive error handling boilerplate.

#### High-level features

<!-- In addition to being performant and safe, Rust also has ergonomic and powerful features
similiar to other high-level languages: -->

* **Expression-oriented** —
  Function bodies[^func-ret-expr], and statements like [`if`][if], [`match`][match], and
  [`loop`][loop] all yield a value. No need for ternary operators and procedural `return`
  statements.

* **Functional** —
  Rust functions can be used as values, passed as arguments, and stored. Closures[^closures]
  can capture variables from their environment and be abstracted over generically.
  Iterator combinators can be used to filter, map, fold, etc. sequencial
  data.[^iter]

* **Immutable by default** –
  In Rust, all variables and references are immutable by default. The [`mut`][mut] keyword
  is used to make the possibility of mutation explicit.

* **Algebraic data types** —
  Structs, tuples, and [`enum`][enum]s with associated data give the programmer
  flexibility and precision when modeling their problem domain. Use pattern
  matching[^pattern-matching] to access values quickly and robustly.

#### High-leverage tools

* **Dependency management** –
  Rust has built-in support for managing dependencies, via the [`cargo`][cargo] command-line
  tool. Use `cargo` to automatically download your dependencies, build your library, run
  tests, and more. Easily use any of the tens of thousands of existing libraries from
  [crates.io](https://crates.io) in your project.

* **Testing** –
  Use [`cargo test`][cargo-test] to run all unit, integration, and doc tests[^doc-tests]
  in your package. Define [unit tests][unit-tests] alongside your library code to ensure
  they stay up to date.

* **Benchmarking** –
  Use [`cargo bench`][cargo-bench] to run benchmarks defined by your package.

### Why `wolfram-library-link`?

TODO

#### Interesting in trying Rust?

The [**Quick Start**](./QuickStart.md) guide contains instructions on how to write a
basic Wolfram LibraryLink program in Rust.

<!--------------->
<!-- Footnotes -->
<!--------------->

[^mem-safe]:
    The [Understanding Ownership][UnderstandingOwnership] chapter of the Rust Book describes
    how the concepts of *ownership* and *borrowing* are central to Rust's memory safety
    guarantees. This [StackOverflow answer](https://stackoverflow.com/a/36137381) contains
    a good informal description of ownership and borrowing.

[^thread-safe]:
    Rust's [`Send`][Send] and [`Sync`][Sync] traits model the properties of data that can
    safely be shared and accessed concurrently in multi-threaded programs. The
    [Fearless Concurrency][FearlessConcurrency] post from the Rust Blog is an accessible
    introduction to how Rust's concepts of *ownership* and *borrowing* lead naturally to
    how thread-safety is modeled.
<!-- The [Send and Sync][Rustonomicon/Send-and-Sync] chapter
of [The Rustonomicon][Rustonomicon] goes into more detail about the `Send`
and `Sync` traits. -->

[^func-ret-expr]: If the last statement in a function is an expression, that value is
    [returned from the function](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values)

[^closures]: See the Rust By Example chapter on
    [Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html).

[^iter]: See the Rust by Example chapter on
    [Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

[^pattern-matching]: Rust has first-class support for pattern matching: [`let`][let],
    [`match`][match], [`if let`][if-let], [`while let`][while-let], function parameters,
    and more all support pattern matching over struct, tuple, array, and enum values. See
    the [Patterns and Matching][PatternsAndMatching] chapter of the Rust Book.

[^doc-tests]: In addition to supporting standard unit- and integration-style tests, Rust
    also supports running code samples that appear in documentation comments as test cases.
    Called [documentation tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html),
    this capability ensures that Rust code samples appearing in your library documentation
    are valid and up to date.

<!----------------->
<!-- Named links -->
<!----------------->

[if]: https://doc.rust-lang.org/std/keyword.if.html
[loop]: https://doc.rust-lang.org/std/keyword.loop.html
[match]: https://doc.rust-lang.org/std/keyword.match.html
[enum]: https://doc.rust-lang.org/std/keyword.enum.html
[mut]: https://doc.rust-lang.org/std/keyword.mut.html
[let]: https://doc.rust-lang.org/std/keyword.let.html

[if-let]: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
[while-let]: https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

[unit-tests]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html#the-anatomy-of-a-test-function

[Send]: https://doc.rust-lang.org/std/marker/trait.Send.html
[Sync]: https://doc.rust-lang.org/std/marker/trait.Sync.html

[FearlessConcurrency]: https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[UnderstandingOwnership]: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
[PatternsAndMatching]: https://doc.rust-lang.org/book/ch18-00-patterns.html
[Rustonomicon/Send-and-Sync]: https://doc.rust-lang.org/nomicon/send-and-sync.html

[cargo]: https://doc.rust-lang.org/cargo/getting-started/first-steps.html
[cargo-test]: https://doc.rust-lang.org/cargo/commands/cargo-test.html
[cargo-bench]: https://doc.rust-lang.org/cargo/commands/cargo-bench.html