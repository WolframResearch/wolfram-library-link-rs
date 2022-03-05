# Functionality Overview

## Function Types

#### Native functions

#### WSTP functions

#### TODO: Expr functions

## WSTP Functions

### Symbol contexts problem

<details>
 <summary>
  Background
 </summary>

 In the Wolfram Language, a symbol is made up of two parts: a context, and a symbol name.
 For example, in the the symbol `` System`Plot ``, `` System` `` is the context, and `Plot`
 is the symbol name. The context denotes a collection of related symbols. For example, all
 symbols that are part of the core Wolfram Language are in the ``"System`"`` context; when
 you start a new Wolfram Language session, the default context that new symbols get created
 in is called ``"Global`"``; and so on. However, you won't often see symbol contexts
 written out explicitly in a Wolfram Language program. Instead, when a symbol name is
 entered, the system looks up that symbol name in the contexts listed in the
 [`$ContextPath`][ref/$ContextPath]: if a symbol with that name exists in one of the
 listed contexts, then the symbol name the user entered resolves to that context.

 So, for example, if the user enters the symbol name `Plot`, and ``"System`"`` is on
 `$ContextPath`, the system deduces the user was referring to the symbol ``System`Plot``.
 In this way, `$ContextPath` allows the user to user shorter symbol names to refer to
 symbols, and avoid having to write out the full context + symbol name as input.

 This shortening also works when printing symbols. For example, doing `ToString[Plot]`
 doesn't return ``"System`Plot"``, but rather just `"Plot"`. And herein lies the problem
 for WSTP functions.
</details>

When an expression is sent across a WSTP link, symbols whose contexts are equal to
[`$Context`][ref/$Context] or on [`$ContextPath`][ref/$ContextPath] will be sent as
strings without the symbol context.

This is a problem because, within Rust code, a symbol name without a context is ambiguous.
If Rust code is expecting to get the symbol ``MyPackage`foo``, but recieves just `foo`
over the WSTP link, there is no easy way for it to tell if that `foo` came from the symbol
it expected, or e.g. ``MyOtherPackage`foo``.

#### The solution

When calling a WSTP function that parses the incoming arguments using the
[`Expr`][crate::expr::Expr] type in some way (e.g. by calling `Link::get_expr()`), use
the following idiom:

```wolfram
(* func: LibraryFunction[_, _, LinkObject, LinkObject] *)

Block[{$Context = "UnusedContext`", $ContextPath = {}},
    func[arg1, arg2, ...]
]
```

Setting `$Context` and `$ContextPath` in this way will force symbols written to the
function `Link` object to explicitly include their context.

### Panic `Failure[..]`s

When a WSTP function [panics][panics], a [`Failure["RustPanic", ...]`][ref/Failure] object
will be returned to the Wolfram Language.

[panics]: https://doc.rust-lang.org/std/macro.panic.html
[ref/Failure]: https://reference.wolfram.com/language/ref/Failure.html

##### Example

```rust
# mod scope {
use wolfram_library_link::{self as wll, wstp::Link};

#[wll::export(wstp)]
fn sqrt(link: &mut Link) {
    let arg_count = link.test_head("List").unwrap();

    if arg_count != 1 {
        panic!("expected 1 argument, got {}", arg_count);
    }

    let arg = link.get_f64().expect("expected Real argument");

    if arg.is_negative() {
        panic!("cannot get the square root of a negative number");
    }

    let value = arg.sqrt();

    link.put_f64(value).unwrap()
}
# }
```

```wolfram
sqrt = LibraryFunctionLoad["...", "sqrt", LinkObject, LinkObject];

sqrt[]

(* Returns:
    Failure["RustPanic", <|
        "MessageTemplate" -> "Rust LibraryLink function panic: `message`",
        "MessageParameters" -> <| "message" -> "expected 1 argument, got 0" |>,
        "SourceLocation" -> "<...>",
        "Backtrace" -> Missing["NotEnabled"]
    >]
*)
```

#### Backtraces

`wolfram-library-link` can optionally capture a stack backtrace for panics that occur
within Rust LibraryLink functions. This behavior is disabled by default, but can be
enabled by setting the `LIBRARY_LINK_RUST_BACKTRACE` environment variable to `"True"`:

```wolfram
SetEnvironment["LIBRARY_LINK_RUST_BACKTRACE" -> "True"]
```

##### Note on panic hooks

`wolfram-library-link` captures information about panics by setting a custom Rust
[panic hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html).


<!----------------->
<!-- Named links -->
<!----------------->

[ref/$Context]: https://reference.wolfram.com/language/ref/$Context.html
[ref/$ContextPath]: https://reference.wolfram.com/language/ref/$ContextPath.html