//! # Which `escape` function to use
//!
//! Generally, if the text goes in an attribute, use [`escape_attribute()`],
//! otherwise, use [`escape_text()`].
//!
//! |                         | `&` | `<` | `>` | `"` | `'` |
//! |-------------------------|:---:|:---:|:---:|:---:|:---:|
//! | [`escape_text()`]       |  ✔  |  ✔  |  ✔  |     |     |
//! | [`escape_attribute()`]  |  ✔  |  ✔  |  ✔  |  ✔  |     |
//! | [`escape_all_quotes()`] |  ✔  |  ✔  |  ✔  |  ✔  |  ✔  |
//!
//! You should almost never need [`escape_all_quotes()`], but is included
//! because sometimes it’s convenient to wrap attribute values in single quotes.
//!
//! # Features
//!
//!   * `unescape`: build [`ENTITIES`] map and provide [`unescape()`]. Enabling
//!     this will add a dependency on [phf][] and may slow builds by a few
//!     seconds.
//!
//! The `escape` functions are all available with no features enabled.
//!
//! [phf]: https://crates.io/crates/phf

#[cfg(test)]
#[macro_use]
pub mod test_helpers;

mod escape;
pub use escape::*;

#[cfg(feature = "unescape")]
mod unescape;
#[cfg(feature = "unescape")]
pub use unescape::*;
