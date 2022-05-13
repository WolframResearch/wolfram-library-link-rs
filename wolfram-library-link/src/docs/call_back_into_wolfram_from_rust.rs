/*!
# How To: Call back into Wolfram from Rust


## Generating Wolfram messages from Rust

Suppose you want to generate a Wolfram [Message\[..\]][ref/Message] from within Rust.

The easiest way to accomplish this is to construct an appropriate Wolfram expression
using the [`Expr`] type, and then use the [`evaluate()`] function to call back into
Wolfram to evaluate that expression.

**Rust**

```rust
# mod scope {
*/
#![doc = include_str!("../../examples/docs/call_back_into_wolfram_from_rust/generate_message.rs")]
/*!
# }
```

**Wolfram**

```wolfram
*/
#![doc = include_str!("../../RustLink/Examples/Docs/CallBackIntoWolframFromRust/GenerateMessage.wlt")]
/*!
```

## Using Print[..] from Rust

*TODO*

*/

use crate::{evaluate, expr::Expr};
