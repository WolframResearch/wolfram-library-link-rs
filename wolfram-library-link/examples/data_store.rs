//! This example demonstrates how the *LibraryLink* "DataStore" type can be used to pass
//! expression-like data structures to and from Rust.
//!
//! The "DataStore" can be manipulated natively from Rust code, and is serialized to
//! an expression structure that looks like:
//!
//! ```wolfram
//! Developer`DataStore[1, "Hello, World!", "key" -> -12.5]
//! ```
//!
//! A "DataStore" contains a sequence of values, which may differ in type, and each
//! value may optionally be named.

use wolfram_library_link::{self as wll, DataStore, DataStoreNodeValue};

//--------------
// string_join()
//--------------

/// Join the strings in a `DataStore`.
///
/// This function may be called by evaluating:
///
/// ```wolfram
/// stringJoin = LibraryFunctionLoad["libdata_store", "string_join", {"DataStore"}, "String"];
///
/// (* Evaluates to: "hello world" *)
/// stringJoin[Developer`DataStore["hello", " ", "world"]]
/// ```
#[wll::export]
fn string_join(store: DataStore) -> String {
    let mut buffer = String::new();

    for node in store.nodes() {
        // If `node.value()` is a string, append it to our string.
        // If `node.value()` is NOT a string, silently skip it.
        if let DataStoreNodeValue::Str(string) = node.value() {
            buffer.push_str(string);
        }
    }

    buffer
}
