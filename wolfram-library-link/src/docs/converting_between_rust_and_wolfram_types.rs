/*!
# How To: Convert Between Rust and Wolfram Types

## Converting Rust types into Wolfram expressions

Suppose you have a type in you Rust program that you want to convert into an equivalent
Wolfram Language expression representation.

For the purpose of this example, we'll assume the Rust structure that you want to
convert into an expression is the following:


```rust
struct Point {
    x: f64,
    y: f64
}
```

and that the desired Wolfram expression representation is a
[`Point`](https://reference.wolfram.com/language/ref/Point.html):

```wolfram
Point[{x, y}]
```


There are two ways to perform this convertion using wolfram-library-link. Both involve
transfering the expression using [WSTP](https://crates.io/crates/wstp), via the
[`#[export(wstp)]`][crate::export#exportwstp] annotation.

### Method #1: Manual WSTP calls

**Rust**

```rust
# mod scope {
*/
#![doc = include_str!("../../examples/docs/convert/manual_wstp.rs")]
/*!
# }
```

**Wolfram**

```wolfram
*/
#![doc = include_str!("../../RustLink/Examples/Docs/Convert/ManualWstp.wlt")]
/*!
```

### Method #2: Convert to [`Expr`]

In this method, instead of passing our `Point[{x, y}]` expression incrementally using
individual WSTP function calls, the `Point` expression is constructed using the [`Expr`]
type.

**Rust**

```rust
# mod scope {
*/
#![doc = include_str!("../../examples/docs/convert/using_expr.rs")]
/*!
# }
```

**Wolfram**

```wolfram
*/
#![doc = include_str!("../../RustLink/Examples/Docs/Convert/UsingExpr.wlt")]
/*!
```

##
*/

use crate::expr::Expr;
