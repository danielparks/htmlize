# Change log

All notable changes to this project will be documented in this file.

## main branch

### Breaking changes

* Hid `unescape()` behind `unescape` feature. This allows users to avoid the
  dependency on [phf][] and the build dependency on [serde_json][], which cuts
  build times on my machine by more than 90% (from 6.2 seconds to 0.5 seconds).

### Improvements

* Switched to the [phf_codegen][] crate instead of using the `phf_map!` macro.
  On my machine, this cuts build time by about 25% (~2 seconds).
* Pre-allocated the output buffer for `unescape()`, which generally improves
  performance slightly.
* Clarified documentation of `ENTITIES` to indicate that it’s a `Map`, not just
  a collection of tuples.

[phf]: https://crates.io/crates/phf
[phf_codegen]: https://crates.io/crates/phf_codegen
[serde_json]: https://crates.io/crates/serde_json

## Release 0.5.1 (2022-12-13)

### Bugfixes

* Switched from [assertify][] (deprecated) to [assert2][] for testing.
* Fixed typo of in docs: “entries.json” should have been “entities.json”.
* Fixed formatting and lints.
* Added this change log.

[assertify]: https://crates.io/crates/assertify
[assert2]: https://crates.io/crates/assert2
