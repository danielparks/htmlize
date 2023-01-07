//! # What each `escape` function escapes
//!
//! The `escape` functions should cover most cases where you need to safely
//! embed a string in HTML. Generally, if the text goes in an attribute, use
//! [`escape_attribute`], otherwise use [`escape_text`].
//!
//! The differences between the functions are more exactly summed up below.
//!
//! Character | Entity   | `escape_text` | `escape_attribute` | `escape_all_quotes`
//! ----------|----------|---------------|--------------------|--------------------
//! `&`       | `&amp;`  | ✔             | ✔                  | ✔
//! `<`       | `&lt;`   | ✔             | ✔                  | ✔
//! `>`       | `&gt;`   | ✔             | ✔                  | ✔
//! `"`       | `&quot;` |               | ✔                  | ✔
//! `'`       | `&apos;` |               |                    | ✔
//!
//! **Note:** These are not sufficient to escape strings embedded in comments.

#[cfg(test)]
#[macro_use]
pub mod test_helpers;

mod escape;
pub use escape::*;

mod unescape;
pub use unescape::*;
