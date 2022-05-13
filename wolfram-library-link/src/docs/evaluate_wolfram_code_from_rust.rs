/*!
# How To: Evaluate Wolfram code from Rust


## Generating Wolfram messages from Rust

Suppose you want to generate a Wolfram [`Message[..]`][ref/Message] from within Rust.

The easiest way to accomplish this is to construct an appropriate Wolfram expression
using the [`Expr`] type, and then use the [`evaluate()`] function to call back into
Wolfram to evaluate that expression.

[ref/Message]: https://reference.wolfram.com/language/ref/Message.html

**Rust**

```rust
# mod scope {
*/
#![doc = include_str!("../../examples/docs/evaluate_wolfram_code_from_rust/generate_message.rs")]
/*!
# }
```

**Wolfram**

```wolfram
*/
#![doc = include_str!("../../RustLink/Examples/Docs/EvaluateWolframCodeFromRust/GenerateMessage.wlt")]
/*!
```

## Using Print[..] from Rust

*TODO*

*/

use crate::{evaluate, expr::Expr};
