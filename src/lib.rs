//! # Which `escape` function to use
//!
//! Generally, if the text goes in an attribute, use [`escape_attribute()`],
//! otherwise, use [`escape_text()`].
//!
//! |                         | `&` | `<` | `>` | `"` | `'` |
//! |-------------------------|:---:|:---:|:---:|:---:|:---:|
//! | [`escape_text()`]       |  ✓  |  ✓  |  ✓  |     |     |
//! | [`escape_attribute()`]  |  ✓  |  ✓  |  ✓  |  ✓  |     |
//! | [`escape_all_quotes()`] |  ✓  |  ✓  |  ✓  |  ✓  |  ✓  |
//!
//! You should almost never need [`escape_all_quotes()`], but is included
//! because sometimes it’s convenient to wrap attribute values in single quotes.
//!
//! # Which `unescape` function to use
//!
//! [`unescape()`] is probably fine for most uses. To be strictly correct, you
//! should use [`unescape_attribute()`] for attribute values.
//!
//! [`unescape_in()`] handles either depending on the value of the `context`
//! parameter. See its documentation for a discussion of the differences between
//! expanding attribute values and general text.
//!
//! [`unescape_bytes_in()`] is just like [`unescape_in()`] except that it works
//! on `[u8]` rather than strings.
//!
//! # Features
//!
//!   * `unescape`: build [`ENTITIES`] map and provide [`unescape()`]. Enabling
//!     this will add a dependency on [phf] and may slow builds by a few
//!     seconds.
//!   * `iai`: enable [iai] benchmarks. This should only be used when running
//!     benchmarks. See the [Benchmarks section in the README][benchmarks].
//!
//! The `escape` functions are all available with no features enabled.
//!
//! # Minimum supported Rust version
//!
//! Currently the minimum supported Rust version (MSRV) is **1.60**. Future
//! increases in the MSRV will require a major version bump.
//!
//! [phf]: https://crates.io/crates/phf
//! [iai]: https://crates.io/crates/iai
//! [benchmarks]: https://github.com/danielparks/htmlize#benchmarks

mod escape;
pub use escape::*;

#[cfg(feature = "unescape")]
mod unescape;
#[cfg(feature = "unescape")]
pub use unescape::*;
